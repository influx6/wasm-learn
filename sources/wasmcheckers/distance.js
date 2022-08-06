/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/
fetch('./distance.wasm').then(response =>
  response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
  console.log("Loaded wasm module - distance");
  instance = results.instance;
  console.log("instance", instance);

  console.log("Calling  distance");
  console.log("d(2,2) = ", instance.exports.distance(2,2));
  console.log("d(4,2) = ", instance.exports.distance(4,2));
  console.log("d(2,4) = ", instance.exports.distance(2,4));
  console.log("d(2,6) = ", instance.exports.distance(2,6));

  console.log("Calling valid distance");
  console.log("v(2,2) = ", instance.exports.validJumpDistance(2,2));
  console.log("v(4,2) = ", instance.exports.validJumpDistance(4,2));
  console.log("v(2,4) = ", instance.exports.validJumpDistance(2,4));
  console.log("v(2,6) = ", instance.exports.validJumpDistance(2,6));
});
