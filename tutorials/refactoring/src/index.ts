import { statement } from "./invoice";
const plays = require("./playes.json");
const invoices = require("./invoices.json");

console.log(JSON.stringify(plays, null, "  "));
console.log(JSON.stringify(invoices, null, "  "));
statement(invoices, plays);
