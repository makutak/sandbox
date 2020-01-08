interface Play {
  name: string;
  type: string;
}

interface Plays {
  subject: Play;
}

interface Performance {
  playID: string;
  audience: number;
}

export interface Invoice {
  customer: string;
  performances: Performance[];
}

export function statement(invoice: Invoice, plays: Plays): string {
  let totalAmount: number = 0;
  let volumeCredits: number = 0;
  let result = `Statement for ${invoice.customer}\n`;

  const format = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2
  }).format;

  for (let perf of invoice.performances) {
    const play = plays[perf.playID];
    let thisAmount: number = amountFor(perf, play);

    volumeCredits += Math.max(perf.audience - 30, 0);

    if ("comedy" === play.type) {
      volumeCredits += Math.floor(perf.audience / 5);
    }
    result += ` ${play.name}: ${format(thisAmount / 100)} (${perf.audience} seats) \n`;
    totalAmount += thisAmount;
  }
  result += `Amount owed is ${format(totalAmount / 100)}\n`;
  result += `You earned ${volumeCredits} credits\n`;
  return result;
}

export function amountFor(perf: Performance, play: Play): number {
  let thisAmount: number = 0;

  switch (play.type) {
    case "tragedy":
      thisAmount = 40000;
      if (perf.audience > 30) {
        thisAmount += 1000 * (perf.audience - 30);
      }
      break;
    case "comedy":
      thisAmount = 30000;
      if (perf.audience > 20) {
        thisAmount += 10000 + 500 * (perf.audience - 20);
      }
      thisAmount += 300 * perf.audience;
      break;
    default:
      throw new Error(`unknown type: ${play.type}`);
  }

  return thisAmount;
}
