// rollup.config.js
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

export default {
  input: "main.js",
  output: { file: "./pkg/bundle.js", format: "iife" },
  plugins: [
    serve(), // index.html should be in root of project
    livereload({
        watch: './src',
    })
  ]
};
