package main

import (
	"bufio"
	"encoding/csv"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"time"
)

func main() {
	benchedAlgorithms := []string{
		"bubble-sort",
		"selection-sort",
		"insertion-sort",
	}
	var benchdata []BenchData

	file, err := os.Open("test.data")
	if err != nil {
		log.Fatal("reading test data failed")
	}
	filescanner := bufio.NewScanner(file)
	filescanner.Split(bufio.ScanLines)
	for filescanner.Scan() {
		DistributeBench(&benchdata, benchedAlgorithms, filescanner.Text())
	}

	if err := WriteToCsv(benchdata); err != nil {
		log.Fatal("write to csv failed")
	}
}

func DistributeBench(data *[]BenchData, algolist []string, valuestr string) {
	values := strings.Split(valuestr, ",")

	dataqueue := make(chan BenchData, len(algolist))
	defer close(dataqueue)

	for _, algo := range algolist {
		bench := NewBencher(algo, values)
		go bench.Bench(dataqueue)
	}

	for received := 0; received < len(algolist); received++ {
		datum := <-dataqueue
		*data = append(*data, datum)
	}
}

type BenchData struct {
	algoname string
	insize   int
	elapsed  float64
}

func WriteToCsv(data []BenchData) error {
	filename := fmt.Sprintf("./results/%s.csv", time.Now().Format("2006-01-02_15:04:05"))
	csvfile, err := os.Create(filename)

	w := csv.NewWriter(csvfile)
	defer w.Flush()

	csvdata := benchToCsv(data)
	w.WriteAll(csvdata)

	return err
}

func benchToCsv(data []BenchData) [][]string {
	csvdata := [][]string{
		{"algorithm", "n", "exectime (ms)"},
	}
	for _, bench := range data {
		record := []string{
			bench.algoname,
			strconv.Itoa(bench.insize),
			strconv.FormatFloat(bench.elapsed, 'f', -1, 32),
		}
		csvdata = append(csvdata, record)
	}
	return csvdata
}

type Benchmark interface {
	Bench(algoname string, queue chan<- BenchData)
}

type AlgoBench struct {
	algoname string
	binary   exec.Cmd
	input    []string
}

func contains[T comparable](slice []T, element T) bool {
	for _, value := range slice {
		if element == value {
			return true
		}
	}
	return false
}

func NewBencher(algoname string, input []string) AlgoBench {
	binpath := "./algorithms/target/debug/rustalgo"
	// join all inputs to comma-separated str
	valuestr := strings.Join(input, ",")

	algorithms := []string{
		"bubble-sort",
		"selection-sort",
		"insertion-sort",
		"merge-sort",
		"quick-sort",
		"heap-sort",
	}

	if !contains(algorithms, algoname) {
		log.Fatalf("%s is not a valid algorithm", algoname)
	}

	return AlgoBench{
		algoname: algoname,
		binary:   *exec.Command(binpath, algoname, valuestr),
		input:    input,
	}
}

func (q *AlgoBench) Bench(queue chan<- BenchData) {
	insize := len(q.input)

	start := time.Now()
	if err := q.binary.Run(); err != nil {
		log.Fatalf("%s crashed with input size %d", q.algoname, insize)
	}
	elapsed := time.Since(start)

	queue <- BenchData{
		algoname: q.algoname,
		insize:   insize,
		elapsed:  elapsed.Seconds() * 1000,
	}
}
