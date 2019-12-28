"use strict";

interface Rectangle {
  height: number;
  width: number;
}

const sum: number = 1 + 1;
console.log("sum:", sum);

const str: string = "Hello" + ", " + "Docker" + "!!";
console.log("str: ", str);

const rect: Rectangle = { height: 50, width: 50 };
console.dir(rect);
