CREATE TABLE IF NOT EXISTS `user`
(
	user_id BIGINT PRIMARY KEY AUTO_INCREMENT,
	code VARCHAR(255) DEFAULT "",
	created_at DATETIME DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS `term`
(
	term_id BIGINT PRIMARY KEY AUTO_INCREMENT,
	path VARCHAR(255) NOT NULL DEFAULT "",
	term VARCHAR(255) NOT NULL DEFAULT "",
	content TEXT DEFAULT "",
	created_at DATETIME DEFAULT NOW(),
	updated_at DATETIME DEFAULT NOW(),
	deleted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS `user_result`
(
	user_result_id BIGINT PRIMARY KEY AUTO_INCREMENT,
	user_id BIGINT DEFAULT 0,
	created_at DATETIME DEFAULT NOW(),
	FOREIGN KEY (user_id) references user(user_id)
);

CREATE TABLE IF NOT EXISTS `user_answer`
(
	user_answer_id BIGINT PRIMARY KEY AUTO_INCREMENT,
	user_result_id BIGINT DEFAULT 0,
	term_id BIGINT DEFAULT 0,
	is_correct BOOLEAN DEFAULT FALSE,
	created_at DATETIME DEFAULT NOW(),
	FOREIGN KEY (term_id) references term(term_id),
	FOREIGN KEY (user_result_id) references user_result(user_result_id)
);
