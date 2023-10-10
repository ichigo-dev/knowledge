ALTER TABLE `user_result`
DROP FOREIGN KEY
	`user_result_ibfk_1`;

ALTER TABLE `user_answer`
DROP FOREIGN KEY
	`user_answer_ibfk_1`;

ALTER TABLE `user_answer`
DROP FOREIGN KEY
	`user_answer_ibfk_2`;

ALTER TABLE `user_result`
ADD FOREIGN KEY (`user_id`) REFERENCES `user` (`user_id`)
	ON DELETE SET NULL;

ALTER TABLE `user_answer`
ADD FOREIGN KEY (`term_id`) REFERENCES `term` (`term_id`)
	ON DELETE SET NULL,
ADD FOREIGN KEY (`user_result_id`) REFERENCES `user_result` (`user_result_id`)
	ON DELETE SET NULL;
