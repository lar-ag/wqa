use super::error::CanError;
use serde_derive::{Deserialize,Serialize};



/// Pin Assignment PL4 ( Outputs Byte 0 )
///
/// Bit Pin  No.   Function
/// 0    1  IN00  Digital Input DC 24V
/// 1    2  IN01  Digital Input DC 24V
/// 2    3  IN02  Digital Input DC 24V
/// 3    4  IN03  Digital Input DC 24V
/// 4    5  IN04  Digital Input DC 24V
/// 5    6  IN05  Digital Input DC 24V
/// 6    7  IN06  Digital Input DC 24V
/// 7    8  IN07  Digital Input DC 24V
/// 8    9  IN10  Digital Input DC 24V
/// 9    10 IN11  Digital Input DC 24V
/// 10   11 IN12  Digital Input DC 24V
/// 11   12 IN13  Digital Input DC 24V
/// 12   13 IN14  Digital Input DC 24V
/// 13   14 IN15  Digital Input DC 24V
/// 14   15 IN16  Digital Input DC 24V
/// 15   16 IN17  Digital Input DC 24V







pub async fn get_in01(bus: CanBus) -> Result<bool,
//
