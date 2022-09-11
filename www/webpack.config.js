const path=require("path");
module.exports={
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "bootstrap.js"
    },
    mode:"development",
    module: {
        rules: [
          {
            test: /\.tsx?$/,
            use: 'ts-loader',
            exclude: /node_modules/,
          },
        ],
      },
      resolve: {
        extensions: ['.tsx', '.ts', '.js'],
      },
      //A bit unsure about this part under
    //   plugins: [
    //     new CopyWebpackPlugin({
    //         patterns: [
    //             {from: ".index.html", to: "./" }
    //         ]
    //     })
    //   ]
}