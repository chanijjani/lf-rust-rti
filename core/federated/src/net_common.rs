/**
 * @file
 * @author Edward A. Lee (eal@berkeley.edu)
 * @author Soroush Bateni (soroush@utdallas.edu)
 * @author Erling Jellum (erling.r.jellum@ntnu.no)
 * @author Chadlia Jerad (chadlia.jerad@ensi-uma.tn)
 * @author Hokeun Kim (hokeun@asu.edu)
 * @author Chanhee Lee (chanheel@asu.edu)
 * @copyright (c) 2020-2023, The University of California at Berkeley
 * License in [BSD 2-clause](..)
 * @brief Declarations for runtime infrastructure (RTI) for distributed Lingua Franca programs.
 * This file extends enclave.h with RTI features that are specific to federations and are not
 * used by scheduling enclaves.
 */
use crate::tag::{Instant, Microstep};
/**
 * Size of the buffer used for messages sent between federates.
 * This is used by both the federates and the rti, so message lengths
 * should generally match.
 */
pub const FED_COM_BUFFER_SIZE: usize = 256;

/**
 * Delay the start of all federates by this amount.
 * FIXME: More.
 * FIXME: Should use the latency estimates that were
 * acquired during initial clock synchronization.
 */
pub const DELAY_START: i64 = 1;

pub const MSG_TYPE_TIMESTAMP_LENGTH: usize = 1 + std::mem::size_of::<i64>();

/**
 * Byte identifying a stop request. This message is first sent to the RTI by a federate
 * that would like to stop execution at the specified tag. The RTI will forward
 * the MSG_TYPE_StopRequest to all other federates. Those federates will either agree to
 * the requested tag or propose a larger tag. The RTI will collect all proposed
 * tags and broadcast the largest of those to all federates. All federates
 * will then be expected to stop at the granted tag.
 *
 * The next 8 bytes will be the timestamp.
 * The next 4 bytes will be the microstep.
 *
 * NOTE: The RTI may reply with a larger tag than the one specified in this message.
 * It has to be that way because if any federate can send a MSG_TYPE_StopRequest message
 * that specifies the stop time on all other federates, then every federate
 * depends on every other federate and time cannot be advanced.
 * Hence, the actual stop time may be nondeterministic.
 *
 * If, on the other hand, the federate requesting the stop is upstream of every
 * other federate, then it should be possible to respect its requested stop tag.
 */
pub const MSG_TYPE_STOP_REQUEST_LENGTH: usize =
    1 + std::mem::size_of::<Instant>() + std::mem::size_of::<Microstep>();
// #[macro_export]
// macro_rules! ENCODE_StopGranted {
//     (buffer, time, microstep) => {
//         buffer[0] = MsgType::StopGranted;
//         NetUtil::encode_int64(time, &buffer, 1);
//         assert(microstep >= 0);
//         NetUtil::encode_int32(microstep as i32, &buffer, 1 + std::mem::size_of::<Instant>());
//     }
// }

/**
 * Byte indicating a federate's reply to a MSG_TYPE_StopRequest that was sent
 * by the RTI. The payload is a proposed stop tag that is at least as large
 * as the one sent to the federate in a MSG_TYPE_StopRequest message.
 *
 * The next 8 bytes will be the timestamp.
 * The next 4 bytes will be the microstep.
 */
pub const MSG_TYPE_STOP_REQUEST_REPLY_LENGTH: usize =
    1 + std::mem::size_of::<Instant>() + std::mem::size_of::<Microstep>();

/**
 * Byte sent by the RTI indicating that the stop request from some federate
 * has been granted. The payload is the tag at which all federates have
 * agreed that they can stop.
 * The next 8 bytes will be the time at which the federates will stop. *
 * The next 4 bytes will be the microstep at which the federates will stop..
 */
pub const MSG_TYPE_STOP_GRANTED_LENGTH: usize =
    1 + std::mem::size_of::<Instant>() + std::mem::size_of::<Microstep>();

pub const MSG_TYPE_NEIGHBOR_STRUCTURE_HEADER_SIZE: i32 = 9;

#[derive(Debug, PartialEq)]
pub enum MsgType {
    Reject,
    FedIds,
    Timestamp,
    Resign,
    TaggedMessage,
    NextEventTag,
    TagAdvanceGrant,
    PropositionalTagAdvanceGrant,
    LatestTagComplete,
    StopRequest,
    StopRequestReply,
    StopGranted,
    AddressQuery,
    AddressAdvertisement,
    P2pSendingFedId,
    P2pTaggedMessage,
    ClockSyncT1,
    ClockSyncT3,
    ClockSyncT4,
    ClockSyncCodedProbe,
    PortAbsent,
    NeighborStructure,
    Ignore,
    UdpPort,
    Ack,
}

impl MsgType {
    pub fn to_byte(&self) -> u8 {
        match self {
            MsgType::Reject => 0,
            MsgType::FedIds => 1,
            MsgType::Timestamp => 2,
            MsgType::Resign => 4,
            MsgType::TaggedMessage => 5,
            MsgType::NextEventTag => 6,
            MsgType::TagAdvanceGrant => 7,
            MsgType::PropositionalTagAdvanceGrant => 8,
            MsgType::LatestTagComplete => 9,
            MsgType::StopRequest => 10,
            MsgType::StopRequestReply => 11,
            MsgType::StopGranted => 12,
            MsgType::AddressQuery => 13,
            MsgType::AddressAdvertisement => 14,
            MsgType::P2pSendingFedId => 15,
            MsgType::P2pTaggedMessage => 17,
            MsgType::ClockSyncT1 => 19,
            MsgType::ClockSyncT3 => 20,
            MsgType::ClockSyncT4 => 21,
            MsgType::ClockSyncCodedProbe => 22,
            MsgType::PortAbsent => 23,
            MsgType::NeighborStructure => 24,
            MsgType::Ignore => 250,
            MsgType::UdpPort => 254,
            MsgType::Ack => 255,
        }
    }

    pub fn to_msg_type(val: u8) -> MsgType {
        match val {
            2 => MsgType::Timestamp,
            4 => MsgType::Resign,
            5 => MsgType::TaggedMessage,
            6 => MsgType::NextEventTag,
            8 => MsgType::PropositionalTagAdvanceGrant,
            9 => MsgType::LatestTagComplete,
            10 => MsgType::StopRequest,
            11 => MsgType::StopRequestReply,
            12 => MsgType::StopGranted,
            13 => MsgType::AddressQuery,
            14 => MsgType::AddressAdvertisement,
            19 => MsgType::ClockSyncT1,
            20 => MsgType::ClockSyncT3,
            21 => MsgType::ClockSyncT4,
            22 => MsgType::ClockSyncCodedProbe,
            23 => MsgType::PortAbsent,
            _ => MsgType::Ignore,
        }
    }
}

/////////////////////////////////////////////
//// Rejection codes

/**
 * These codes are sent in a MsgType::Reject message.
 * They are limited to one byte (uchar).
 */
pub enum ErrType {
    FederationIdDoesNotMatch,
    FederateIdInUse,
    FederateIdOutOfRange,
    UnexpectedMessage,
    WrongServer,
}

impl ErrType {
    pub fn to_byte(&self) -> u8 {
        match self {
            ErrType::FederationIdDoesNotMatch => 1,
            ErrType::FederateIdInUse => 2,
            ErrType::FederateIdOutOfRange => 3,
            ErrType::UnexpectedMessage => 4,
            ErrType::WrongServer => 5,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum SocketType {
    TCP,
    UDP,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_byte_reject_positive() {
        assert!(MsgType::Reject.to_byte() == 0);
    }

    #[test]
    fn test_to_byte_fed_ids_positive() {
        assert!(MsgType::FedIds.to_byte() == 1);
    }

    #[test]
    fn test_to_byte_timestamp_positive() {
        assert!(MsgType::Timestamp.to_byte() == 2);
    }

    #[test]
    fn test_to_byte_resign_positive() {
        assert!(MsgType::Resign.to_byte() == 4);
    }

    #[test]
    fn test_to_byte_tagged_message_positive() {
        assert!(MsgType::TaggedMessage.to_byte() == 5);
    }

    #[test]
    fn test_to_byte_next_event_tag_positive() {
        assert!(MsgType::NextEventTag.to_byte() == 6);
    }

    #[test]
    fn test_to_byte_tag_advance_grant_positive() {
        assert!(MsgType::TagAdvanceGrant.to_byte() == 7);
    }

    #[test]
    fn test_to_byte_propositional_tag_advance_grant_positive() {
        assert!(MsgType::PropositionalTagAdvanceGrant.to_byte() == 8);
    }

    #[test]
    fn test_to_byte_logical_tag_complete_positive() {
        assert!(MsgType::LatestTagComplete.to_byte() == 9);
    }

    #[test]
    fn test_to_byte_stop_request_positive() {
        assert!(MsgType::StopRequest.to_byte() == 10);
    }

    #[test]
    fn test_to_byte_stop_request_reply_positive() {
        assert!(MsgType::StopRequestReply.to_byte() == 11);
    }

    #[test]
    fn test_to_byte_stop_granted_positive() {
        assert!(MsgType::StopGranted.to_byte() == 12);
    }

    #[test]
    fn test_to_byte_address_query_positive() {
        assert!(MsgType::AddressQuery.to_byte() == 13);
    }

    #[test]
    fn test_to_byte_p2p_sending_fed_id_positive() {
        assert!(MsgType::P2pSendingFedId.to_byte() == 15);
    }

    #[test]
    fn test_to_byte_p2p_tagged_message_positive() {
        assert!(MsgType::P2pTaggedMessage.to_byte() == 17);
    }

    #[test]
    fn test_to_byte_port_absent_positive() {
        assert!(MsgType::PortAbsent.to_byte() == 23);
    }

    #[test]
    fn test_to_byte_neighbor_structure_positive() {
        assert!(MsgType::NeighborStructure.to_byte() == 24);
    }

    #[test]
    fn test_to_byte_ignore_positive() {
        assert!(MsgType::Ignore.to_byte() == 250);
    }

    #[test]
    fn test_to_byte_udp_port_positive() {
        assert!(MsgType::UdpPort.to_byte() == 254);
    }

    #[test]
    fn test_to_byte_ack_positive() {
        assert!(MsgType::Ack.to_byte() == 255);
    }

    #[test]
    fn test_to_msg_type_timestamp_positive() {
        assert!(MsgType::to_msg_type(2) == MsgType::Timestamp);
    }

    #[test]
    fn test_to_msg_type_resign_positive() {
        assert!(MsgType::to_msg_type(4) == MsgType::Resign);
    }

    #[test]
    fn test_to_msg_type_tagged_message_positive() {
        assert!(MsgType::to_msg_type(5) == MsgType::TaggedMessage);
    }

    #[test]
    fn test_to_msg_type_next_event_tag_positive() {
        assert!(MsgType::to_msg_type(6) == MsgType::NextEventTag);
    }

    #[test]
    fn test_to_msg_type_propositional_tag_advance_grant_positive() {
        assert!(MsgType::to_msg_type(8) == MsgType::PropositionalTagAdvanceGrant);
    }

    #[test]
    fn test_to_msg_type_latest_tag_complete_positive() {
        assert!(MsgType::to_msg_type(9) == MsgType::LatestTagComplete);
    }

    #[test]
    fn test_to_msg_type_stop_request_positive() {
        assert!(MsgType::to_msg_type(10) == MsgType::StopRequest);
    }

    #[test]
    fn test_to_msg_type_stop_request_reply_positive() {
        assert!(MsgType::to_msg_type(11) == MsgType::StopRequestReply);
    }

    #[test]
    fn test_to_msg_type_stop_granted_positive() {
        assert!(MsgType::to_msg_type(12) == MsgType::StopGranted);
    }

    #[test]
    fn test_to_msg_type_address_query_positive() {
        assert!(MsgType::to_msg_type(13) == MsgType::AddressQuery);
    }

    #[test]
    fn test_to_msg_type_port_absent_positive() {
        assert!(MsgType::to_msg_type(23) == MsgType::PortAbsent);
    }

    #[test]
    fn test_to_msg_type_ignore_positive() {
        assert!(MsgType::to_msg_type(0) == MsgType::Ignore);
    }

    #[test]
    fn test_to_byte_federation_id_does_not_match_positive() {
        assert!(ErrType::FederationIdDoesNotMatch.to_byte() == 1);
    }

    #[test]
    fn test_to_byte_federate_id_in_use_positive() {
        assert!(ErrType::FederateIdInUse.to_byte() == 2);
    }

    #[test]
    fn test_to_byte_federate_id_out_of_range_positive() {
        assert!(ErrType::FederateIdOutOfRange.to_byte() == 3);
    }

    #[test]
    fn test_to_byte_unexpected_message_positive() {
        assert!(ErrType::UnexpectedMessage.to_byte() == 4);
    }

    #[test]
    fn test_to_byte_wrong_server_positive() {
        assert!(ErrType::WrongServer.to_byte() == 5);
    }
}
