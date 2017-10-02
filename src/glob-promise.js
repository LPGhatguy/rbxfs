import glob from "glob";

export default function globPromise(files: string, options: Object = {}) {
	return new Promise((resolve, reject) => {
		glob(files, options, (err, files) => {
			if (err) {
				return reject(err);
			}

			resolve(files);
		});
	});
}