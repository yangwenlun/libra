use crate::libra_channel::MessageQueue;
use crate::message_queues::{PerValidatorQueue, QueueStyle};
use libra_types::account_address::AccountAddress;
use libra_types::account_address::ADDRESS_LENGTH;

/// This represents a proposal message from a validator
#[derive(Debug, PartialEq)]
struct ProposalMsg {
    msg: String,
}

/// This represents a vote message from a validator
#[derive(Debug, PartialEq)]
struct VoteMsg {
    msg: String,
}

#[test]
fn test_fifo() {
    let mut q = PerValidatorQueue::new(QueueStyle::FIFO, 3);
    let validator = AccountAddress::new([0u8; ADDRESS_LENGTH]);

    // Test order
    q.push(
        validator,
        ProposalMsg {
            msg: "msg1".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg2".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg3".to_string(),
        },
    );
    assert_eq!(q.pop().unwrap().msg, "msg1".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg2".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg3".to_string());

    // Test max queue size
    q.push(
        validator,
        ProposalMsg {
            msg: "msg1".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg2".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg3".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg4".to_string(),
        },
    );
    assert_eq!(q.pop().unwrap().msg, "msg1".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg2".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg3".to_string());
    assert_eq!(q.pop(), None);
}

#[test]
fn test_lifo() {
    let mut q = PerValidatorQueue::new(QueueStyle::LIFO, 3);
    let validator = AccountAddress::new([0u8; ADDRESS_LENGTH]);

    // Test order
    q.push(
        validator,
        ProposalMsg {
            msg: "msg1".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg2".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg3".to_string(),
        },
    );
    assert_eq!(q.pop().unwrap().msg, "msg3".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg2".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg1".to_string());

    // Test max queue size
    q.push(
        validator,
        ProposalMsg {
            msg: "msg1".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg2".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg3".to_string(),
        },
    );
    q.push(
        validator,
        ProposalMsg {
            msg: "msg4".to_string(),
        },
    );
    assert_eq!(q.pop().unwrap().msg, "msg4".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg3".to_string());
    assert_eq!(q.pop().unwrap().msg, "msg2".to_string());
    assert_eq!(q.pop(), None);
}

#[test]
fn test_fifo_round_robin() {
    let mut q = PerValidatorQueue::new(QueueStyle::FIFO, 3);
    let validator1 = AccountAddress::new([0u8; ADDRESS_LENGTH]);
    let validator2 = AccountAddress::new([1u8; ADDRESS_LENGTH]);
    let validator3 = AccountAddress::new([2u8; ADDRESS_LENGTH]);

    q.push(
        validator1,
        ProposalMsg {
            msg: "validator1_msg1".to_string(),
        },
    );
    q.push(
        validator1,
        ProposalMsg {
            msg: "validator1_msg2".to_string(),
        },
    );
    q.push(
        validator1,
        ProposalMsg {
            msg: "validator1_msg3".to_string(),
        },
    );
    q.push(
        validator2,
        ProposalMsg {
            msg: "validator2_msg1".to_string(),
        },
    );
    q.push(
        validator3,
        ProposalMsg {
            msg: "validator3_msg1".to_string(),
        },
    );

    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator1_msg1".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator2_msg1".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator3_msg1".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator1_msg2".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator1_msg3".to_string(),
        }
    );
    assert_eq!(q.pop(), None);
}

#[test]
fn test_lifo_round_robin() {
    let mut q = PerValidatorQueue::new(QueueStyle::LIFO, 3);
    let validator1 = AccountAddress::new([0u8; ADDRESS_LENGTH]);
    let validator2 = AccountAddress::new([1u8; ADDRESS_LENGTH]);
    let validator3 = AccountAddress::new([2u8; ADDRESS_LENGTH]);

    q.push(
        validator1,
        ProposalMsg {
            msg: "validator1_msg1".to_string(),
        },
    );
    q.push(
        validator1,
        ProposalMsg {
            msg: "validator1_msg2".to_string(),
        },
    );
    q.push(
        validator1,
        ProposalMsg {
            msg: "validator1_msg3".to_string(),
        },
    );
    q.push(
        validator2,
        ProposalMsg {
            msg: "validator2_msg1".to_string(),
        },
    );
    q.push(
        validator3,
        ProposalMsg {
            msg: "validator3_msg1".to_string(),
        },
    );

    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator1_msg3".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator2_msg1".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator3_msg1".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator1_msg2".to_string(),
        }
    );
    assert_eq!(
        q.pop().unwrap(),
        ProposalMsg {
            msg: "validator1_msg1".to_string(),
        }
    );
    assert_eq!(q.pop(), None);
}
