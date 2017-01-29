// @flow

import { relative } from "path";

import gulp from "gulp";
import babel from "gulp-babel";
import watch from "gulp-watch";
import nodemon from "nodemon";

const tasks = {};

tasks.build = (files?) => {
	if (files == null) {
		files = "src/**/*.js";
		console.log("Building all files");
	} else {
		console.log(`Building ${ relative(__dirname, files) }`);
	}

	return gulp.src(files)
		.pipe(babel())
		.pipe(gulp.dest("lib"));
};

tasks.watch = () => {
	watch("src", (change) => tasks.build(change.path));
};

tasks.dev = () => {
	tasks.watch();

	return tasks.build();
};

for (const name of Object.keys(tasks)) {
	gulp.task(name, tasks[name]);
}

gulp.task("default", tasks.build);