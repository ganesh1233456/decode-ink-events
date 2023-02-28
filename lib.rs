#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod event_test {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    #[ink(storage)]
    pub struct EventTest {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    #[ink(event)]
    pub struct MyEvent {
        #[ink(topic)]
        value: bool,
    }

    impl EventTest {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(MyEvent { value: self.value });
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use ink::env::test::EmittedEvent;

    //     use super::*;

    //     type Event = <EventTest as ::ink::reflect::ContractEventBase>::Type;

    //     fn decode_events(emitted_events: Vec<EmittedEvent>) -> Vec<Event> {
    //         emitted_events
    //             .into_iter()
    //             .map(|event| {
    //                 <Event as scale::Decode>::decode(&mut &event.data[..]).expect("Invalid data")
    //             })
    //             .collect()
    //     }

    //     #[ink::test]
    //     fn default_works() {
    //         let event_test = EventTest::default();
    //         assert_eq!(event_test.get(), false);
    //     }

    //     #[ink::test]
    //     fn it_works() {
    //         let mut event_test = EventTest::new(false);
    //         assert_eq!(event_test.get(), false);
    //         event_test.flip();

    //         let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
    //         let decode_events = decode_events(emitted_events);

    //         match &decode_events[0] {
    //             Event::MyEvent(MyEvent { value }) => assert_eq!(*value, true),
    //             _ => (),
    //         }
    //         assert_eq!(event_test.get(), true);
    //     }
    // }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
