use std::vec;

enum CoinKind {
    Ten,
    Fifty,
    OneHandred,
    FiveHundred,
}

enum BillKind {
    OneThousand,
}

type CoinNumber = i32;
type BillNumber = i32;

enum Event {
    ThrowInCoin(CoinKind, CoinNumber),
    ThrowInBill(BillKind, BillNumber),
}

struct VendineMachine {
    events: vec::Vec<Event>,
}

trait VendineMachineBehavior {
    fn write_event_log(&mut self, event: Event) -> &mut Self;
    fn throw_in_coin(&mut self, coin: CoinKind, coin_number: CoinNumber) -> &mut Self;
    fn throw_in_bill(&mut self, bill: BillKind, bill_number: BillNumber) -> &mut Self;
    fn total_entry_amount(&mut self) -> i32;
}

impl VendineMachineBehavior for VendineMachine {
    fn write_event_log(&mut self, event: Event) -> &mut Self {
        self.events.push(event);
        self
    }

    fn throw_in_coin(&mut self, coin: CoinKind, coin_number: CoinNumber) -> &mut Self {
        self.write_event_log(Event::ThrowInCoin(coin, coin_number))
    }

    fn throw_in_bill(&mut self, bill: BillKind, bill_number: BillNumber) -> &mut Self {
        self.write_event_log(Event::ThrowInBill(bill, bill_number))
    }
    fn total_entry_amount(&mut self) -> i32 {
        let values_iter = self.events.iter().map(|event| match event {
            Event::ThrowInCoin(coin_kind, n) => {
                let coin_value = match coin_kind {
                    CoinKind::Ten => 10,
                    CoinKind::Fifty => 50,
                    CoinKind::OneHandred => 100,
                    CoinKind::FiveHundred => 500,
                };
                coin_value * n
            }
            Event::ThrowInBill(bill_kind, n) => {
                let bill_value = match bill_kind {
                    BillKind::OneThousand => 1000,
                };
                bill_value * n
            }
        });
        values_iter.sum()
    }
}

fn main() {
    let mut vending_machine = VendineMachine { events: vec![] };
    println!(
        "{}",
        vending_machine
            .throw_in_coin(CoinKind::Ten, 1)
            .throw_in_coin(CoinKind::Fifty, 1)
            .throw_in_coin(CoinKind::OneHandred, 1)
            .throw_in_coin(CoinKind::FiveHundred, 1)
            .throw_in_bill(BillKind::OneThousand, 1)
            .total_entry_amount()
    );
}
