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

interface Invoice {
  customer: string;
  performances: Performance[];
}

export function statement(invoice: Invoice, plays: Plays): void {
  console.log("this is statement function");
}
