CREATE TABLE IF NOT EXISTS term
(
	term_id INT PRIMARY KEY,
	term VARCHAR(255) NOT NULL,
	content TEXT
);

CREATE TABLE IF NOT EXISTS mask
(
	mask_id INT PRIMARY KEY,
	term_id INT,
	mask VARCHAR(255) NOT NULL,
	FOREIGN KEY (term_id) references term(term_id)
);

CREATE TABLE IF NOT EXISTS quiz_result
(
	quiz_result_id INT PRIMARY KEY,
	quiz_cnt INT,
	correct_cnt INT,
	incorrect_cnt INT,
	created_at DATETIME DEFAULT (DATETIME('now', 'localtime'))
);

CREATE TABLE IF NOT EXISTS answer
(
	answer_id INT PRIMARY KEY,
	quiz_result_id INT,
	term_id INT,
	is_correct BOOLEAN,
	FOREIGN KEY (quiz_result_id) references quiz_result(quiz_result_id),
	FOREIGN KEY (term_id) references term(term_id)
);
