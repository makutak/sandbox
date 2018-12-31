"use strict";

interface Rectangle {
  height: number;
  width: number;
}

class Human {
  height: number;
  weight: number;

  constructor(height: number, weight: number) {
    this.height = height;
    this.weight = weight;
  }

  public calcBmi(): number {
    const h = this.height / 100;
    return (this.weight / (h * h));
  }
}

const sum: number = 1 + 1;
console.log("sum:", sum);

const str: string = "Hello" + ", " + "Docker" + "!!";
console.log("str: ", str);

const rect: Rectangle = { height: 50, width: 50 };
console.dir(rect);

const human = new Human(170, 55);
console.log("your BMI is: ", human.calcBmi());

console.log("hello");
