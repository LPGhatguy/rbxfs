const glob = require("glob");

function globPromise(files, options) {
	return new Promise((resolve, reject) => {
		glob(files, options, (err, files) => {
			if (err) {
				return reject(err);
			}

			resolve(files);
		});
	});
}

module.exports = globPromise;