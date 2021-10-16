package main

import (
	"encoding/csv"
	"flag"
	"fmt"
	"os"
	"time"
)

type problem struct {
	q string
	a string
}

func parseLines(lines [][]string) []problem {
	probs := make([]problem, len(lines))

	for i, line := range lines {
		probs[i] = problem{
			q: line[0],
			a: line[1],
		}
	}
	return probs
}

func exit(msg string) {
	fmt.Println(msg)
	os.Exit(1)
}

func main() {
	csvFilename := flag.String("csv", "../input/problems.csv", "CSV file in format 'Q,A'")
	timeLimit := flag.Int("limit", 30, "Time limit for quiz in seconds")
	flag.Parse()

	csvfile, err := os.Open(*csvFilename)
	if err != nil {
		exit(fmt.Sprintf("Error opening csv file: %s", err))
	}
	csvr := csv.NewReader(csvfile)

	lines, err := csvr.ReadAll()
	if err != nil {
		exit("Error reading csv file")
	}

	probs := parseLines(lines)

	fmt.Println("Banana quiz about to start. Press enter when ready.")
	fmt.Scanf("%s\n")

	timer := time.NewTimer(time.Duration(*timeLimit) * time.Second)
	correct_ans := 0
	for _, p := range probs {
		fmt.Printf("What banana? %s\n", p.q)
		bananaCh := make(chan string)
		go func() {
			var banana string
			fmt.Scanf("%s\n", &banana)
			bananaCh <- banana
		}()

		select {
		case <-timer.C:
			fmt.Printf("Correct answers: %d/%d\n", correct_ans, len(probs))
			return
		case banana := <-bananaCh:
			fmt.Printf("Your Banana: %s, Correct Banana: %s\n", banana, p.a)

			if banana != p.a {
				fmt.Println("BAD BANANA!")
				break
			}
			fmt.Println("good banana!")
			fmt.Println("------------")
			correct_ans++
		}
	}
	fmt.Printf("Correct answers: %d/%d\n", correct_ans, len(probs))
}
