use std::fs::File;
use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    question_id: i32,
    question_text: String,
    answers: Vec<Answer>,
}

impl Question {
    pub fn new(question_id: i32, question_text: String, answers: Vec<Answer>) -> Self {
        Self {
            question_id,
            question_text,
            answers,
        }
    }
    pub fn to_flat_record(&self) -> String {
        let mut record = vec![self.question_id.to_string(), self.question_text.clone()];
        for answer in &self.answers {
            record.push(answer.answer_text.clone());
            record.push(answer.next_question.to_string());
        }
        record.join(", ")
    }
    pub fn show_text(&self) -> &String {
        &self.question_text
    }
    pub fn get_answers(&self) -> Vec<Answer> {
        self.answers.clone()
    }
    pub fn get_next_question_number(&self, answer_number: usize) -> usize {
        self.answers[answer_number].get_next_question_number()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    answer_text: String,
    next_question: usize,
}

impl Answer {
    pub fn new(answer_text: String, next_question: usize) -> Self {
        Self {
            answer_text,
            next_question,
        }
    }
    pub fn show_text(&self) -> &String {
        &self.answer_text
    }
    pub fn get_next_question_number(&self) -> usize {
        self.next_question
    }
}

fn parse_csv_row(row: &str) -> Question {
    // Split the row by commas and collect fields into a vector
    let fields: Vec<&str> = row.split(',').map(|s| s.trim()).collect();

    // Parse the question ID and question text
    let question_id: i32 = fields[0]
        .parse()
        .expect("The question id is not a an integer");
    let question_text = fields[1].to_string();

    // Parse answers in pairs of (answer_text, next_question)
    let mut answers = Vec::new();
    let mut i = 2;
    while i < fields.len() {
        if let (Some(answer_text), Some(next_question_str)) = (fields.get(i), fields.get(i + 1)) {
            let next_question = next_question_str
                .parse()
                .expect("Next question id is not an integer");
            answers.push(Answer::new(answer_text.to_string(), next_question));
        }
        i += 2; // Move to the next pair of fields
    }

    Question::new(question_id, question_text, answers)
}

pub fn read_questions_from_csv(file_path: &str) -> Vec<Question> {
    let mut questions = Vec::new();

    // Open the file and create a buffered reader
    let file = File::open(file_path).expect("Didn't find file.");
    let reader = io::BufReader::new(file);

    // Iterate over each line in the CSV file
    for line in reader.lines() {
        let line = line.expect("Couldn't read the lines.");
        questions.push(parse_csv_row(&line));
    }

    questions
}
