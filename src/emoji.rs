#[derive(Debug)]
pub struct Emoji {
    pub unicode: char,
    pub name: String,
}

pub fn get_emoji_list() -> Vec<Emoji> {
    vec![
        Emoji {
            unicode: '\u{0023}',
            name: "NUMBER SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{002A}',
            name: "ASTERISK".to_string(),
        },
        Emoji {
            unicode: '\u{0030}',
            name: "DIGIT ZERO".to_string(),
        },
        Emoji {
            unicode: '\u{0031}',
            name: "DIGIT ONE".to_string(),
        },
        Emoji {
            unicode: '\u{0032}',
            name: "DIGIT TWO".to_string(),
        },
        Emoji {
            unicode: '\u{0033}',
            name: "DIGIT THREE".to_string(),
        },
        Emoji {
            unicode: '\u{0034}',
            name: "DIGIT FOUR".to_string(),
        },
        Emoji {
            unicode: '\u{0035}',
            name: "DIGIT FIVE".to_string(),
        },
        Emoji {
            unicode: '\u{0036}',
            name: "DIGIT SIX".to_string(),
        },
        Emoji {
            unicode: '\u{0037}',
            name: "DIGIT SEVEN".to_string(),
        },
        Emoji {
            unicode: '\u{0038}',
            name: "DIGIT EIGHT".to_string(),
        },
        Emoji {
            unicode: '\u{0039}',
            name: "DIGIT NINE".to_string(),
        },
        Emoji {
            unicode: '\u{00A9}',
            name: "COPYRIGHT SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{00AE}',
            name: "REGISTERED SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{203C}',
            name: "DOUBLE EXCLAMATION MARK".to_string(),
        },
        Emoji {
            unicode: '\u{2049}',
            name: "EXCLAMATION QUESTION MARK".to_string(),
        },
        Emoji {
            unicode: '\u{2122}',
            name: "TRADE MARK SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2139}',
            name: "INFORMATION SOURCE".to_string(),
        },
        Emoji {
            unicode: '\u{2194}',
            name: "LEFT RIGHT ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2195}',
            name: "UP DOWN ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2196}',
            name: "NORTH WEST ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2197}',
            name: "NORTH EAST ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2198}',
            name: "SOUTH EAST ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2199}',
            name: "SOUTH WEST ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{21A9}',
            name: "LEFTWARDS ARROW WITH HOOK".to_string(),
        },
        Emoji {
            unicode: '\u{21AA}',
            name: "RIGHTWARDS ARROW WITH HOOK".to_string(),
        },
        Emoji {
            unicode: '\u{231A}',
            name: "WATCH".to_string(),
        },
        Emoji {
            unicode: '\u{231B}',
            name: "HOURGLASS".to_string(),
        },
        Emoji {
            unicode: '\u{2328}',
            name: "KEYBOARD".to_string(),
        },
        Emoji {
            unicode: '\u{23CF}',
            name: "EJECT SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{23E9}',
            name: "BLACK RIGHT-POINTING DOUBLE TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{23EA}',
            name: "BLACK LEFT-POINTING DOUBLE TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{23EB}',
            name: "BLACK UP-POINTING DOUBLE TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{23EC}',
            name: "BLACK DOWN-POINTING DOUBLE TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{23ED}',
            name: "BLACK RIGHT-POINTING DOUBLE TRIANGLE WITH VERTICAL BAR".to_string(),
        },
        Emoji {
            unicode: '\u{23EE}',
            name: "BLACK LEFT-POINTING DOUBLE TRIANGLE WITH VERTICAL BAR".to_string(),
        },
        Emoji {
            unicode: '\u{23EF}',
            name: "BLACK RIGHT-POINTING TRIANGLE WITH DOUBLE VERTICAL BAR".to_string(),
        },
        Emoji {
            unicode: '\u{23F0}',
            name: "ALARM CLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{23F1}',
            name: "STOPWATCH".to_string(),
        },
        Emoji {
            unicode: '\u{23F2}',
            name: "TIMER CLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{23F3}',
            name: "HOURGLASS WITH FLOWING SAND".to_string(),
        },
        Emoji {
            unicode: '\u{23F8}',
            name: "DOUBLE VERTICAL BAR".to_string(),
        },
        Emoji {
            unicode: '\u{23F9}',
            name: "BLACK SQUARE FOR STOP".to_string(),
        },
        Emoji {
            unicode: '\u{23FA}',
            name: "BLACK CIRCLE FOR RECORD".to_string(),
        },
        Emoji {
            unicode: '\u{24C2}',
            name: "CIRCLED LATIN CAPITAL LETTER M".to_string(),
        },
        Emoji {
            unicode: '\u{25AA}',
            name: "BLACK SMALL SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{25AB}',
            name: "WHITE SMALL SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{25B6}',
            name: "BLACK RIGHT-POINTING TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{25C0}',
            name: "BLACK LEFT-POINTING TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{25FB}',
            name: "WHITE MEDIUM SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{25FC}',
            name: "BLACK MEDIUM SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{25FD}',
            name: "WHITE MEDIUM SMALL SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{25FE}',
            name: "BLACK MEDIUM SMALL SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{2600}',
            name: "BLACK SUN WITH RAYS".to_string(),
        },
        Emoji {
            unicode: '\u{2601}',
            name: "CLOUD".to_string(),
        },
        Emoji {
            unicode: '\u{2602}',
            name: "UMBRELLA".to_string(),
        },
        Emoji {
            unicode: '\u{2603}',
            name: "SNOWMAN".to_string(),
        },
        Emoji {
            unicode: '\u{2604}',
            name: "COMET".to_string(),
        },
        Emoji {
            unicode: '\u{260E}',
            name: "BLACK TELEPHONE".to_string(),
        },
        Emoji {
            unicode: '\u{2611}',
            name: "BALLOT BOX WITH CHECK".to_string(),
        },
        Emoji {
            unicode: '\u{2614}',
            name: "UMBRELLA WITH RAIN DROPS".to_string(),
        },
        Emoji {
            unicode: '\u{2615}',
            name: "HOT BEVERAGE".to_string(),
        },
        Emoji {
            unicode: '\u{2618}',
            name: "SHAMROCK".to_string(),
        },
        Emoji {
            unicode: '\u{261D}',
            name: "WHITE UP POINTING INDEX".to_string(),
        },
        Emoji {
            unicode: '\u{2620}',
            name: "SKULL AND CROSSBONES".to_string(),
        },
        Emoji {
            unicode: '\u{2622}',
            name: "RADIOACTIVE SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2623}',
            name: "BIOHAZARD SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2626}',
            name: "ORTHODOX CROSS".to_string(),
        },
        Emoji {
            unicode: '\u{262A}',
            name: "STAR AND CRESCENT".to_string(),
        },
        Emoji {
            unicode: '\u{262E}',
            name: "PEACE SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{262F}',
            name: "YIN YANG".to_string(),
        },
        Emoji {
            unicode: '\u{2638}',
            name: "WHEEL OF DHARMA".to_string(),
        },
        Emoji {
            unicode: '\u{2639}',
            name: "WHITE FROWNING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{263A}',
            name: "WHITE SMILING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{2640}',
            name: "FEMALE SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2642}',
            name: "MALE SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2648}',
            name: "ARIES".to_string(),
        },
        Emoji {
            unicode: '\u{2649}',
            name: "TAURUS".to_string(),
        },
        Emoji {
            unicode: '\u{264A}',
            name: "GEMINI".to_string(),
        },
        Emoji {
            unicode: '\u{264B}',
            name: "CANCER".to_string(),
        },
        Emoji {
            unicode: '\u{264C}',
            name: "LEO".to_string(),
        },
        Emoji {
            unicode: '\u{264D}',
            name: "VIRGO".to_string(),
        },
        Emoji {
            unicode: '\u{264E}',
            name: "LIBRA".to_string(),
        },
        Emoji {
            unicode: '\u{264F}',
            name: "SCORPIUS".to_string(),
        },
        Emoji {
            unicode: '\u{2650}',
            name: "SAGITTARIUS".to_string(),
        },
        Emoji {
            unicode: '\u{2651}',
            name: "CAPRICORN".to_string(),
        },
        Emoji {
            unicode: '\u{2652}',
            name: "AQUARIUS".to_string(),
        },
        Emoji {
            unicode: '\u{2653}',
            name: "PISCES".to_string(),
        },
        Emoji {
            unicode: '\u{265F}',
            name: "CHESS PAWN".to_string(),
        },
        Emoji {
            unicode: '\u{2660}',
            name: "BLACK SPADE SUIT".to_string(),
        },
        Emoji {
            unicode: '\u{2663}',
            name: "BLACK CLUB SUIT".to_string(),
        },
        Emoji {
            unicode: '\u{2665}',
            name: "BLACK HEART SUIT".to_string(),
        },
        Emoji {
            unicode: '\u{2666}',
            name: "BLACK DIAMOND SUIT".to_string(),
        },
        Emoji {
            unicode: '\u{2668}',
            name: "HOT SPRINGS".to_string(),
        },
        Emoji {
            unicode: '\u{267B}',
            name: "BLACK UNIVERSAL RECYCLING SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{267E}',
            name: "INFINITY".to_string(),
        },
        Emoji {
            unicode: '\u{267F}',
            name: "WHEELCHAIR SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{2692}',
            name: "HAMMER AND PICK".to_string(),
        },
        Emoji {
            unicode: '\u{2693}',
            name: "ANCHOR".to_string(),
        },
        Emoji {
            unicode: '\u{2694}',
            name: "CROSSED SWORDS".to_string(),
        },
        Emoji {
            unicode: '\u{2695}',
            name: "STAFF OF AESCULAPIUS".to_string(),
        },
        Emoji {
            unicode: '\u{2696}',
            name: "SCALES".to_string(),
        },
        Emoji {
            unicode: '\u{2697}',
            name: "ALEMBIC".to_string(),
        },
        Emoji {
            unicode: '\u{2699}',
            name: "GEAR".to_string(),
        },
        Emoji {
            unicode: '\u{269B}',
            name: "ATOM SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{269C}',
            name: "FLEUR-DE-LIS".to_string(),
        },
        Emoji {
            unicode: '\u{26A0}',
            name: "WARNING SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{26A1}',
            name: "HIGH VOLTAGE SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{26AA}',
            name: "MEDIUM WHITE CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{26AB}',
            name: "MEDIUM BLACK CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{26B0}',
            name: "COFFIN".to_string(),
        },
        Emoji {
            unicode: '\u{26B1}',
            name: "FUNERAL URN".to_string(),
        },
        Emoji {
            unicode: '\u{26BD}',
            name: "SOCCER BALL".to_string(),
        },
        Emoji {
            unicode: '\u{26BE}',
            name: "BASEBALL".to_string(),
        },
        Emoji {
            unicode: '\u{26C4}',
            name: "SNOWMAN WITHOUT SNOW".to_string(),
        },
        Emoji {
            unicode: '\u{26C5}',
            name: "SUN BEHIND CLOUD".to_string(),
        },
        Emoji {
            unicode: '\u{26C8}',
            name: "THUNDER CLOUD AND RAIN".to_string(),
        },
        Emoji {
            unicode: '\u{26CE}',
            name: "OPHIUCHUS".to_string(),
        },
        Emoji {
            unicode: '\u{26CF}',
            name: "PICK".to_string(),
        },
        Emoji {
            unicode: '\u{26D1}',
            name: "HELMET WITH WHITE CROSS".to_string(),
        },
        Emoji {
            unicode: '\u{26D3}',
            name: "CHAINS".to_string(),
        },
        Emoji {
            unicode: '\u{26D4}',
            name: "NO ENTRY".to_string(),
        },
        Emoji {
            unicode: '\u{26E9}',
            name: "SHINTO SHRINE".to_string(),
        },
        Emoji {
            unicode: '\u{26EA}',
            name: "CHURCH".to_string(),
        },
        Emoji {
            unicode: '\u{26F0}',
            name: "MOUNTAIN".to_string(),
        },
        Emoji {
            unicode: '\u{26F1}',
            name: "UMBRELLA ON GROUND".to_string(),
        },
        Emoji {
            unicode: '\u{26F2}',
            name: "FOUNTAIN".to_string(),
        },
        Emoji {
            unicode: '\u{26F3}',
            name: "FLAG IN HOLE".to_string(),
        },
        Emoji {
            unicode: '\u{26F4}',
            name: "FERRY".to_string(),
        },
        Emoji {
            unicode: '\u{26F5}',
            name: "SAILBOAT".to_string(),
        },
        Emoji {
            unicode: '\u{26F7}',
            name: "SKIER".to_string(),
        },
        Emoji {
            unicode: '\u{26F8}',
            name: "ICE SKATE".to_string(),
        },
        Emoji {
            unicode: '\u{26F9}',
            name: "PERSON WITH BALL".to_string(),
        },
        Emoji {
            unicode: '\u{26FA}',
            name: "TENT".to_string(),
        },
        Emoji {
            unicode: '\u{26FD}',
            name: "FUEL PUMP".to_string(),
        },
        Emoji {
            unicode: '\u{2702}',
            name: "BLACK SCISSORS".to_string(),
        },
        Emoji {
            unicode: '\u{2705}',
            name: "WHITE HEAVY CHECK MARK".to_string(),
        },
        Emoji {
            unicode: '\u{2708}',
            name: "AIRPLANE".to_string(),
        },
        Emoji {
            unicode: '\u{2709}',
            name: "ENVELOPE".to_string(),
        },
        Emoji {
            unicode: '\u{270A}',
            name: "RAISED FIST".to_string(),
        },
        Emoji {
            unicode: '\u{270B}',
            name: "RAISED HAND".to_string(),
        },
        Emoji {
            unicode: '\u{270C}',
            name: "VICTORY HAND".to_string(),
        },
        Emoji {
            unicode: '\u{270D}',
            name: "WRITING HAND".to_string(),
        },
        Emoji {
            unicode: '\u{270F}',
            name: "PENCIL".to_string(),
        },
        Emoji {
            unicode: '\u{2712}',
            name: "BLACK NIB".to_string(),
        },
        Emoji {
            unicode: '\u{2714}',
            name: "HEAVY CHECK MARK".to_string(),
        },
        Emoji {
            unicode: '\u{2716}',
            name: "HEAVY MULTIPLICATION X".to_string(),
        },
        Emoji {
            unicode: '\u{271D}',
            name: "LATIN CROSS".to_string(),
        },
        Emoji {
            unicode: '\u{2721}',
            name: "STAR OF DAVID".to_string(),
        },
        Emoji {
            unicode: '\u{2728}',
            name: "SPARKLES".to_string(),
        },
        Emoji {
            unicode: '\u{2733}',
            name: "EIGHT SPOKED ASTERISK".to_string(),
        },
        Emoji {
            unicode: '\u{2734}',
            name: "EIGHT POINTED BLACK STAR".to_string(),
        },
        Emoji {
            unicode: '\u{2744}',
            name: "SNOWFLAKE".to_string(),
        },
        Emoji {
            unicode: '\u{2747}',
            name: "SPARKLE".to_string(),
        },
        Emoji {
            unicode: '\u{274C}',
            name: "CROSS MARK".to_string(),
        },
        Emoji {
            unicode: '\u{274E}',
            name: "NEGATIVE SQUARED CROSS MARK".to_string(),
        },
        Emoji {
            unicode: '\u{2753}',
            name: "BLACK QUESTION MARK ORNAMENT".to_string(),
        },
        Emoji {
            unicode: '\u{2754}',
            name: "WHITE QUESTION MARK ORNAMENT".to_string(),
        },
        Emoji {
            unicode: '\u{2755}',
            name: "WHITE EXCLAMATION MARK ORNAMENT".to_string(),
        },
        Emoji {
            unicode: '\u{2757}',
            name: "HEAVY EXCLAMATION MARK SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{2763}',
            name: "HEAVY HEART EXCLAMATION MARK ORNAMENT".to_string(),
        },
        Emoji {
            unicode: '\u{2764}',
            name: "HEAVY BLACK HEART".to_string(),
        },
        Emoji {
            unicode: '\u{2795}',
            name: "HEAVY PLUS SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2796}',
            name: "HEAVY MINUS SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{2797}',
            name: "HEAVY DIVISION SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{27A1}',
            name: "BLACK RIGHTWARDS ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{27B0}',
            name: "CURLY LOOP".to_string(),
        },
        Emoji {
            unicode: '\u{27BF}',
            name: "DOUBLE CURLY LOOP".to_string(),
        },
        Emoji {
            unicode: '\u{2934}',
            name: "ARROW POINTING RIGHTWARDS THEN CURVING UPWARDS".to_string(),
        },
        Emoji {
            unicode: '\u{2935}',
            name: "ARROW POINTING RIGHTWARDS THEN CURVING DOWNWARDS".to_string(),
        },
        Emoji {
            unicode: '\u{2B05}',
            name: "LEFTWARDS BLACK ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2B06}',
            name: "UPWARDS BLACK ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2B07}',
            name: "DOWNWARDS BLACK ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{2B1B}',
            name: "BLACK LARGE SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{2B1C}',
            name: "WHITE LARGE SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{2B50}',
            name: "WHITE MEDIUM STAR".to_string(),
        },
        Emoji {
            unicode: '\u{2B55}',
            name: "HEAVY LARGE CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{3030}',
            name: "WAVY DASH".to_string(),
        },
        Emoji {
            unicode: '\u{303D}',
            name: "PART ALTERNATION MARK".to_string(),
        },
        Emoji {
            unicode: '\u{3297}',
            name: "CIRCLED IDEOGRAPH CONGRATULATION".to_string(),
        },
        Emoji {
            unicode: '\u{3299}',
            name: "CIRCLED IDEOGRAPH SECRET".to_string(),
        },
        Emoji {
            unicode: '\u{1F004}',
            name: "MAHJONG TILE RED DRAGON".to_string(),
        },
        Emoji {
            unicode: '\u{1F0CF}',
            name: "PLAYING CARD BLACK JOKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F170}',
            name: "NEGATIVE SQUARED LATIN CAPITAL LETTER A".to_string(),
        },
        Emoji {
            unicode: '\u{1F171}',
            name: "NEGATIVE SQUARED LATIN CAPITAL LETTER B".to_string(),
        },
        Emoji {
            unicode: '\u{1F17E}',
            name: "NEGATIVE SQUARED LATIN CAPITAL LETTER O".to_string(),
        },
        Emoji {
            unicode: '\u{1F17F}',
            name: "NEGATIVE SQUARED LATIN CAPITAL LETTER P".to_string(),
        },
        Emoji {
            unicode: '\u{1F18E}',
            name: "NEGATIVE SQUARED AB".to_string(),
        },
        Emoji {
            unicode: '\u{1F191}',
            name: "SQUARED CL".to_string(),
        },
        Emoji {
            unicode: '\u{1F192}',
            name: "SQUARED COOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F193}',
            name: "SQUARED FREE".to_string(),
        },
        Emoji {
            unicode: '\u{1F194}',
            name: "SQUARED ID".to_string(),
        },
        Emoji {
            unicode: '\u{1F195}',
            name: "SQUARED NEW".to_string(),
        },
        Emoji {
            unicode: '\u{1F196}',
            name: "SQUARED NG".to_string(),
        },
        Emoji {
            unicode: '\u{1F197}',
            name: "SQUARED OK".to_string(),
        },
        Emoji {
            unicode: '\u{1F198}',
            name: "SQUARED SOS".to_string(),
        },
        Emoji {
            unicode: '\u{1F199}',
            name: "SQUARED UP WITH EXCLAMATION MARK".to_string(),
        },
        Emoji {
            unicode: '\u{1F19A}',
            name: "SQUARED VS".to_string(),
        },
        Emoji {
            unicode: '\u{1F1E6}',
            name: "REGIONAL INDICATOR SYMBOL LETTER A".to_string(),
        },
        Emoji {
            unicode: '\u{1F1E7}',
            name: "REGIONAL INDICATOR SYMBOL LETTER B".to_string(),
        },
        Emoji {
            unicode: '\u{1F1E8}',
            name: "REGIONAL INDICATOR SYMBOL LETTER C".to_string(),
        },
        Emoji {
            unicode: '\u{1F1E9}',
            name: "REGIONAL INDICATOR SYMBOL LETTER D".to_string(),
        },
        Emoji {
            unicode: '\u{1F1EA}',
            name: "REGIONAL INDICATOR SYMBOL LETTER E".to_string(),
        },
        Emoji {
            unicode: '\u{1F1EB}',
            name: "REGIONAL INDICATOR SYMBOL LETTER F".to_string(),
        },
        Emoji {
            unicode: '\u{1F1EC}',
            name: "REGIONAL INDICATOR SYMBOL LETTER G".to_string(),
        },
        Emoji {
            unicode: '\u{1F1ED}',
            name: "REGIONAL INDICATOR SYMBOL LETTER H".to_string(),
        },
        Emoji {
            unicode: '\u{1F1EE}',
            name: "REGIONAL INDICATOR SYMBOL LETTER I".to_string(),
        },
        Emoji {
            unicode: '\u{1F1EF}',
            name: "REGIONAL INDICATOR SYMBOL LETTER J".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F0}',
            name: "REGIONAL INDICATOR SYMBOL LETTER K".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F1}',
            name: "REGIONAL INDICATOR SYMBOL LETTER L".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F2}',
            name: "REGIONAL INDICATOR SYMBOL LETTER M".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F3}',
            name: "REGIONAL INDICATOR SYMBOL LETTER N".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F4}',
            name: "REGIONAL INDICATOR SYMBOL LETTER O".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F5}',
            name: "REGIONAL INDICATOR SYMBOL LETTER P".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F6}',
            name: "REGIONAL INDICATOR SYMBOL LETTER Q".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F7}',
            name: "REGIONAL INDICATOR SYMBOL LETTER R".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F8}',
            name: "REGIONAL INDICATOR SYMBOL LETTER S".to_string(),
        },
        Emoji {
            unicode: '\u{1F1F9}',
            name: "REGIONAL INDICATOR SYMBOL LETTER T".to_string(),
        },
        Emoji {
            unicode: '\u{1F1FA}',
            name: "REGIONAL INDICATOR SYMBOL LETTER U".to_string(),
        },
        Emoji {
            unicode: '\u{1F1FB}',
            name: "REGIONAL INDICATOR SYMBOL LETTER V".to_string(),
        },
        Emoji {
            unicode: '\u{1F1FC}',
            name: "REGIONAL INDICATOR SYMBOL LETTER W".to_string(),
        },
        Emoji {
            unicode: '\u{1F1FD}',
            name: "REGIONAL INDICATOR SYMBOL LETTER X".to_string(),
        },
        Emoji {
            unicode: '\u{1F1FE}',
            name: "REGIONAL INDICATOR SYMBOL LETTER Y".to_string(),
        },
        Emoji {
            unicode: '\u{1F1FF}',
            name: "REGIONAL INDICATOR SYMBOL LETTER Z".to_string(),
        },
        Emoji {
            unicode: '\u{1F201}',
            name: "SQUARED KATAKANA KOKO".to_string(),
        },
        Emoji {
            unicode: '\u{1F202}',
            name: "SQUARED KATAKANA SA".to_string(),
        },
        Emoji {
            unicode: '\u{1F21A}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-7121".to_string(),
        },
        Emoji {
            unicode: '\u{1F22F}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-6307".to_string(),
        },
        Emoji {
            unicode: '\u{1F232}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-7981".to_string(),
        },
        Emoji {
            unicode: '\u{1F233}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-7A7A".to_string(),
        },
        Emoji {
            unicode: '\u{1F234}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-5408".to_string(),
        },
        Emoji {
            unicode: '\u{1F235}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-6E80".to_string(),
        },
        Emoji {
            unicode: '\u{1F236}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-6709".to_string(),
        },
        Emoji {
            unicode: '\u{1F237}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-6708".to_string(),
        },
        Emoji {
            unicode: '\u{1F238}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-7533".to_string(),
        },
        Emoji {
            unicode: '\u{1F239}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-5272".to_string(),
        },
        Emoji {
            unicode: '\u{1F23A}',
            name: "SQUARED CJK UNIFIED IDEOGRAPH-55B6".to_string(),
        },
        Emoji {
            unicode: '\u{1F250}',
            name: "CIRCLED IDEOGRAPH ADVANTAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F251}',
            name: "CIRCLED IDEOGRAPH ACCEPT".to_string(),
        },
        Emoji {
            unicode: '\u{1F300}',
            name: "CYCLONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F301}',
            name: "FOGGY".to_string(),
        },
        Emoji {
            unicode: '\u{1F302}',
            name: "CLOSED UMBRELLA".to_string(),
        },
        Emoji {
            unicode: '\u{1F303}',
            name: "NIGHT WITH STARS".to_string(),
        },
        Emoji {
            unicode: '\u{1F304}',
            name: "SUNRISE OVER MOUNTAINS".to_string(),
        },
        Emoji {
            unicode: '\u{1F305}',
            name: "SUNRISE".to_string(),
        },
        Emoji {
            unicode: '\u{1F306}',
            name: "CITYSCAPE AT DUSK".to_string(),
        },
        Emoji {
            unicode: '\u{1F307}',
            name: "SUNSET OVER BUILDINGS".to_string(),
        },
        Emoji {
            unicode: '\u{1F308}',
            name: "RAINBOW".to_string(),
        },
        Emoji {
            unicode: '\u{1F309}',
            name: "BRIDGE AT NIGHT".to_string(),
        },
        Emoji {
            unicode: '\u{1F30A}',
            name: "WATER WAVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F30B}',
            name: "VOLCANO".to_string(),
        },
        Emoji {
            unicode: '\u{1F30C}',
            name: "MILKY WAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F30D}',
            name: "EARTH GLOBE EUROPE-AFRICA".to_string(),
        },
        Emoji {
            unicode: '\u{1F30E}',
            name: "EARTH GLOBE AMERICAS".to_string(),
        },
        Emoji {
            unicode: '\u{1F30F}',
            name: "EARTH GLOBE ASIA-AUSTRALIA".to_string(),
        },
        Emoji {
            unicode: '\u{1F310}',
            name: "GLOBE WITH MERIDIANS".to_string(),
        },
        Emoji {
            unicode: '\u{1F311}',
            name: "NEW MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F312}',
            name: "WAXING CRESCENT MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F313}',
            name: "FIRST QUARTER MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F314}',
            name: "WAXING GIBBOUS MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F315}',
            name: "FULL MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F316}',
            name: "WANING GIBBOUS MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F317}',
            name: "LAST QUARTER MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F318}',
            name: "WANING CRESCENT MOON SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F319}',
            name: "CRESCENT MOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F31A}',
            name: "NEW MOON WITH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F31B}',
            name: "FIRST QUARTER MOON WITH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F31C}',
            name: "LAST QUARTER MOON WITH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F31D}',
            name: "FULL MOON WITH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F31E}',
            name: "SUN WITH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F31F}',
            name: "GLOWING STAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F320}',
            name: "SHOOTING STAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F321}',
            name: "THERMOMETER".to_string(),
        },
        Emoji {
            unicode: '\u{1F324}',
            name: "WHITE SUN WITH SMALL CLOUD".to_string(),
        },
        Emoji {
            unicode: '\u{1F325}',
            name: "WHITE SUN BEHIND CLOUD".to_string(),
        },
        Emoji {
            unicode: '\u{1F326}',
            name: "WHITE SUN BEHIND CLOUD WITH RAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F327}',
            name: "CLOUD WITH RAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F328}',
            name: "CLOUD WITH SNOW".to_string(),
        },
        Emoji {
            unicode: '\u{1F329}',
            name: "CLOUD WITH LIGHTNING".to_string(),
        },
        Emoji {
            unicode: '\u{1F32A}',
            name: "CLOUD WITH TORNADO".to_string(),
        },
        Emoji {
            unicode: '\u{1F32B}',
            name: "FOG".to_string(),
        },
        Emoji {
            unicode: '\u{1F32C}',
            name: "WIND BLOWING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F32D}',
            name: "HOT DOG".to_string(),
        },
        Emoji {
            unicode: '\u{1F32E}',
            name: "TACO".to_string(),
        },
        Emoji {
            unicode: '\u{1F32F}',
            name: "BURRITO".to_string(),
        },
        Emoji {
            unicode: '\u{1F330}',
            name: "CHESTNUT".to_string(),
        },
        Emoji {
            unicode: '\u{1F331}',
            name: "SEEDLING".to_string(),
        },
        Emoji {
            unicode: '\u{1F332}',
            name: "EVERGREEN TREE".to_string(),
        },
        Emoji {
            unicode: '\u{1F333}',
            name: "DECIDUOUS TREE".to_string(),
        },
        Emoji {
            unicode: '\u{1F334}',
            name: "PALM TREE".to_string(),
        },
        Emoji {
            unicode: '\u{1F335}',
            name: "CACTUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F336}',
            name: "HOT PEPPER".to_string(),
        },
        Emoji {
            unicode: '\u{1F337}',
            name: "TULIP".to_string(),
        },
        Emoji {
            unicode: '\u{1F338}',
            name: "CHERRY BLOSSOM".to_string(),
        },
        Emoji {
            unicode: '\u{1F339}',
            name: "ROSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F33A}',
            name: "HIBISCUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F33B}',
            name: "SUNFLOWER".to_string(),
        },
        Emoji {
            unicode: '\u{1F33C}',
            name: "BLOSSOM".to_string(),
        },
        Emoji {
            unicode: '\u{1F33D}',
            name: "EAR OF MAIZE".to_string(),
        },
        Emoji {
            unicode: '\u{1F33E}',
            name: "EAR OF RICE".to_string(),
        },
        Emoji {
            unicode: '\u{1F33F}',
            name: "HERB".to_string(),
        },
        Emoji {
            unicode: '\u{1F340}',
            name: "FOUR LEAF CLOVER".to_string(),
        },
        Emoji {
            unicode: '\u{1F341}',
            name: "MAPLE LEAF".to_string(),
        },
        Emoji {
            unicode: '\u{1F342}',
            name: "FALLEN LEAF".to_string(),
        },
        Emoji {
            unicode: '\u{1F343}',
            name: "LEAF FLUTTERING IN WIND".to_string(),
        },
        Emoji {
            unicode: '\u{1F344}',
            name: "MUSHROOM".to_string(),
        },
        Emoji {
            unicode: '\u{1F345}',
            name: "TOMATO".to_string(),
        },
        Emoji {
            unicode: '\u{1F346}',
            name: "AUBERGINE".to_string(),
        },
        Emoji {
            unicode: '\u{1F347}',
            name: "GRAPES".to_string(),
        },
        Emoji {
            unicode: '\u{1F348}',
            name: "MELON".to_string(),
        },
        Emoji {
            unicode: '\u{1F349}',
            name: "WATERMELON".to_string(),
        },
        Emoji {
            unicode: '\u{1F34A}',
            name: "TANGERINE".to_string(),
        },
        Emoji {
            unicode: '\u{1F34B}',
            name: "LEMON".to_string(),
        },
        Emoji {
            unicode: '\u{1F34C}',
            name: "BANANA".to_string(),
        },
        Emoji {
            unicode: '\u{1F34D}',
            name: "PINEAPPLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F34E}',
            name: "RED APPLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F34F}',
            name: "GREEN APPLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F350}',
            name: "PEAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F351}',
            name: "PEACH".to_string(),
        },
        Emoji {
            unicode: '\u{1F352}',
            name: "CHERRIES".to_string(),
        },
        Emoji {
            unicode: '\u{1F353}',
            name: "STRAWBERRY".to_string(),
        },
        Emoji {
            unicode: '\u{1F354}',
            name: "HAMBURGER".to_string(),
        },
        Emoji {
            unicode: '\u{1F355}',
            name: "SLICE OF PIZZA".to_string(),
        },
        Emoji {
            unicode: '\u{1F356}',
            name: "MEAT ON BONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F357}',
            name: "POULTRY LEG".to_string(),
        },
        Emoji {
            unicode: '\u{1F358}',
            name: "RICE CRACKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F359}',
            name: "RICE BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F35A}',
            name: "COOKED RICE".to_string(),
        },
        Emoji {
            unicode: '\u{1F35B}',
            name: "CURRY AND RICE".to_string(),
        },
        Emoji {
            unicode: '\u{1F35C}',
            name: "STEAMING BOWL".to_string(),
        },
        Emoji {
            unicode: '\u{1F35D}',
            name: "SPAGHETTI".to_string(),
        },
        Emoji {
            unicode: '\u{1F35E}',
            name: "BREAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F35F}',
            name: "FRENCH FRIES".to_string(),
        },
        Emoji {
            unicode: '\u{1F360}',
            name: "ROASTED SWEET POTATO".to_string(),
        },
        Emoji {
            unicode: '\u{1F361}',
            name: "DANGO".to_string(),
        },
        Emoji {
            unicode: '\u{1F362}',
            name: "ODEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F363}',
            name: "SUSHI".to_string(),
        },
        Emoji {
            unicode: '\u{1F364}',
            name: "FRIED SHRIMP".to_string(),
        },
        Emoji {
            unicode: '\u{1F365}',
            name: "FISH CAKE WITH SWIRL DESIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F366}',
            name: "SOFT ICE CREAM".to_string(),
        },
        Emoji {
            unicode: '\u{1F367}',
            name: "SHAVED ICE".to_string(),
        },
        Emoji {
            unicode: '\u{1F368}',
            name: "ICE CREAM".to_string(),
        },
        Emoji {
            unicode: '\u{1F369}',
            name: "DOUGHNUT".to_string(),
        },
        Emoji {
            unicode: '\u{1F36A}',
            name: "COOKIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F36B}',
            name: "CHOCOLATE BAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F36C}',
            name: "CANDY".to_string(),
        },
        Emoji {
            unicode: '\u{1F36D}',
            name: "LOLLIPOP".to_string(),
        },
        Emoji {
            unicode: '\u{1F36E}',
            name: "CUSTARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F36F}',
            name: "HONEY POT".to_string(),
        },
        Emoji {
            unicode: '\u{1F370}',
            name: "SHORTCAKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F371}',
            name: "BENTO BOX".to_string(),
        },
        Emoji {
            unicode: '\u{1F372}',
            name: "POT OF FOOD".to_string(),
        },
        Emoji {
            unicode: '\u{1F373}',
            name: "COOKING".to_string(),
        },
        Emoji {
            unicode: '\u{1F374}',
            name: "FORK AND KNIFE".to_string(),
        },
        Emoji {
            unicode: '\u{1F375}',
            name: "TEACUP WITHOUT HANDLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F376}',
            name: "SAKE BOTTLE AND CUP".to_string(),
        },
        Emoji {
            unicode: '\u{1F377}',
            name: "WINE GLASS".to_string(),
        },
        Emoji {
            unicode: '\u{1F378}',
            name: "COCKTAIL GLASS".to_string(),
        },
        Emoji {
            unicode: '\u{1F379}',
            name: "TROPICAL DRINK".to_string(),
        },
        Emoji {
            unicode: '\u{1F37A}',
            name: "BEER MUG".to_string(),
        },
        Emoji {
            unicode: '\u{1F37B}',
            name: "CLINKING BEER MUGS".to_string(),
        },
        Emoji {
            unicode: '\u{1F37C}',
            name: "BABY BOTTLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F37D}',
            name: "FORK AND KNIFE WITH PLATE".to_string(),
        },
        Emoji {
            unicode: '\u{1F37E}',
            name: "BOTTLE WITH POPPING CORK".to_string(),
        },
        Emoji {
            unicode: '\u{1F37F}',
            name: "POPCORN".to_string(),
        },
        Emoji {
            unicode: '\u{1F380}',
            name: "RIBBON".to_string(),
        },
        Emoji {
            unicode: '\u{1F381}',
            name: "WRAPPED PRESENT".to_string(),
        },
        Emoji {
            unicode: '\u{1F382}',
            name: "BIRTHDAY CAKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F383}',
            name: "JACK-O-LANTERN".to_string(),
        },
        Emoji {
            unicode: '\u{1F384}',
            name: "CHRISTMAS TREE".to_string(),
        },
        Emoji {
            unicode: '\u{1F385}',
            name: "FATHER CHRISTMAS".to_string(),
        },
        Emoji {
            unicode: '\u{1F386}',
            name: "FIREWORKS".to_string(),
        },
        Emoji {
            unicode: '\u{1F387}',
            name: "FIREWORK SPARKLER".to_string(),
        },
        Emoji {
            unicode: '\u{1F388}',
            name: "BALLOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F389}',
            name: "PARTY POPPER".to_string(),
        },
        Emoji {
            unicode: '\u{1F38A}',
            name: "CONFETTI BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F38B}',
            name: "TANABATA TREE".to_string(),
        },
        Emoji {
            unicode: '\u{1F38C}',
            name: "CROSSED FLAGS".to_string(),
        },
        Emoji {
            unicode: '\u{1F38D}',
            name: "PINE DECORATION".to_string(),
        },
        Emoji {
            unicode: '\u{1F38E}',
            name: "JAPANESE DOLLS".to_string(),
        },
        Emoji {
            unicode: '\u{1F38F}',
            name: "CARP STREAMER".to_string(),
        },
        Emoji {
            unicode: '\u{1F390}',
            name: "WIND CHIME".to_string(),
        },
        Emoji {
            unicode: '\u{1F391}',
            name: "MOON VIEWING CEREMONY".to_string(),
        },
        Emoji {
            unicode: '\u{1F392}',
            name: "SCHOOL SATCHEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F393}',
            name: "GRADUATION CAP".to_string(),
        },
        Emoji {
            unicode: '\u{1F396}',
            name: "MILITARY MEDAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F397}',
            name: "REMINDER RIBBON".to_string(),
        },
        Emoji {
            unicode: '\u{1F399}',
            name: "STUDIO MICROPHONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F39A}',
            name: "LEVEL SLIDER".to_string(),
        },
        Emoji {
            unicode: '\u{1F39B}',
            name: "CONTROL KNOBS".to_string(),
        },
        Emoji {
            unicode: '\u{1F39E}',
            name: "FILM FRAMES".to_string(),
        },
        Emoji {
            unicode: '\u{1F39F}',
            name: "ADMISSION TICKETS".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A0}',
            name: "CAROUSEL HORSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A1}',
            name: "FERRIS WHEEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A2}',
            name: "ROLLER COASTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A3}',
            name: "FISHING POLE AND FISH".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A4}',
            name: "MICROPHONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A5}',
            name: "MOVIE CAMERA".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A6}',
            name: "CINEMA".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A7}',
            name: "HEADPHONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A8}',
            name: "ARTIST PALETTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3A9}',
            name: "TOP HAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F3AA}',
            name: "CIRCUS TENT".to_string(),
        },
        Emoji {
            unicode: '\u{1F3AB}',
            name: "TICKET".to_string(),
        },
        Emoji {
            unicode: '\u{1F3AC}',
            name: "CLAPPER BOARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F3AD}',
            name: "PERFORMING ARTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F3AE}',
            name: "VIDEO GAME".to_string(),
        },
        Emoji {
            unicode: '\u{1F3AF}',
            name: "DIRECT HIT".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B0}',
            name: "SLOT MACHINE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B1}',
            name: "BILLIARDS".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B2}',
            name: "GAME DIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B3}',
            name: "BOWLING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B4}',
            name: "FLOWER PLAYING CARDS".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B5}',
            name: "MUSICAL NOTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B6}',
            name: "MULTIPLE MUSICAL NOTES".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B7}',
            name: "SAXOPHONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B8}',
            name: "GUITAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F3B9}',
            name: "MUSICAL KEYBOARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F3BA}',
            name: "TRUMPET".to_string(),
        },
        Emoji {
            unicode: '\u{1F3BB}',
            name: "VIOLIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F3BC}',
            name: "MUSICAL SCORE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3BD}',
            name: "RUNNING SHIRT WITH SASH".to_string(),
        },
        Emoji {
            unicode: '\u{1F3BE}',
            name: "TENNIS RACQUET AND BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3BF}',
            name: "SKI AND SKI BOOT".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C0}',
            name: "BASKETBALL AND HOOP".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C1}',
            name: "CHEQUERED FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C2}',
            name: "SNOWBOARDER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C3}',
            name: "RUNNER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C4}',
            name: "SURFER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C5}',
            name: "SPORTS MEDAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C6}',
            name: "TROPHY".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C7}',
            name: "HORSE RACING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C8}',
            name: "AMERICAN FOOTBALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3C9}',
            name: "RUGBY FOOTBALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3CA}',
            name: "SWIMMER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3CB}',
            name: "WEIGHT LIFTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3CC}',
            name: "GOLFER".to_string(),
        },
        Emoji {
            unicode: '\u{1F3CD}',
            name: "RACING MOTORCYCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3CE}',
            name: "RACING CAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F3CF}',
            name: "CRICKET BAT AND BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D0}',
            name: "VOLLEYBALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D1}',
            name: "FIELD HOCKEY STICK AND BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D2}',
            name: "ICE HOCKEY STICK AND PUCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D3}',
            name: "TABLE TENNIS PADDLE AND BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D4}',
            name: "SNOW CAPPED MOUNTAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D5}',
            name: "CAMPING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D6}',
            name: "BEACH WITH UMBRELLA".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D7}',
            name: "BUILDING CONSTRUCTION".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D8}',
            name: "HOUSE BUILDINGS".to_string(),
        },
        Emoji {
            unicode: '\u{1F3D9}',
            name: "CITYSCAPE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3DA}',
            name: "DERELICT HOUSE BUILDING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3DB}',
            name: "CLASSICAL BUILDING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3DC}',
            name: "DESERT".to_string(),
        },
        Emoji {
            unicode: '\u{1F3DD}',
            name: "DESERT ISLAND".to_string(),
        },
        Emoji {
            unicode: '\u{1F3DE}',
            name: "NATIONAL PARK".to_string(),
        },
        Emoji {
            unicode: '\u{1F3DF}',
            name: "STADIUM".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E0}',
            name: "HOUSE BUILDING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E1}',
            name: "HOUSE WITH GARDEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E2}',
            name: "OFFICE BUILDING".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E3}',
            name: "JAPANESE POST OFFICE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E4}',
            name: "EUROPEAN POST OFFICE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E5}',
            name: "HOSPITAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E6}',
            name: "BANK".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E7}',
            name: "AUTOMATED TELLER MACHINE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E8}',
            name: "HOTEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3E9}',
            name: "LOVE HOTEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3EA}',
            name: "CONVENIENCE STORE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3EB}',
            name: "SCHOOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3EC}',
            name: "DEPARTMENT STORE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3ED}',
            name: "FACTORY".to_string(),
        },
        Emoji {
            unicode: '\u{1F3EE}',
            name: "IZAKAYA LANTERN".to_string(),
        },
        Emoji {
            unicode: '\u{1F3EF}',
            name: "JAPANESE CASTLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F0}',
            name: "EUROPEAN CASTLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F3}',
            name: "WAVING WHITE FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F4}',
            name: "WAVING BLACK FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F5}',
            name: "ROSETTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F7}',
            name: "LABEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F8}',
            name: "BADMINTON RACQUET AND SHUTTLECOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F3F9}',
            name: "BOW AND ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{1F3FA}',
            name: "AMPHORA".to_string(),
        },
        Emoji {
            unicode: '\u{1F3FB}',
            name: "EMOJI MODIFIER FITZPATRICK TYPE-1-2".to_string(),
        },
        Emoji {
            unicode: '\u{1F3FC}',
            name: "EMOJI MODIFIER FITZPATRICK TYPE-3".to_string(),
        },
        Emoji {
            unicode: '\u{1F3FD}',
            name: "EMOJI MODIFIER FITZPATRICK TYPE-4".to_string(),
        },
        Emoji {
            unicode: '\u{1F3FE}',
            name: "EMOJI MODIFIER FITZPATRICK TYPE-5".to_string(),
        },
        Emoji {
            unicode: '\u{1F3FF}',
            name: "EMOJI MODIFIER FITZPATRICK TYPE-6".to_string(),
        },
        Emoji {
            unicode: '\u{1F400}',
            name: "RAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F401}',
            name: "MOUSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F402}',
            name: "OX".to_string(),
        },
        Emoji {
            unicode: '\u{1F403}',
            name: "WATER BUFFALO".to_string(),
        },
        Emoji {
            unicode: '\u{1F404}',
            name: "COW".to_string(),
        },
        Emoji {
            unicode: '\u{1F405}',
            name: "TIGER".to_string(),
        },
        Emoji {
            unicode: '\u{1F406}',
            name: "LEOPARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F407}',
            name: "RABBIT".to_string(),
        },
        Emoji {
            unicode: '\u{1F408}',
            name: "CAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F409}',
            name: "DRAGON".to_string(),
        },
        Emoji {
            unicode: '\u{1F40A}',
            name: "CROCODILE".to_string(),
        },
        Emoji {
            unicode: '\u{1F40B}',
            name: "WHALE".to_string(),
        },
        Emoji {
            unicode: '\u{1F40C}',
            name: "SNAIL".to_string(),
        },
        Emoji {
            unicode: '\u{1F40D}',
            name: "SNAKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F40E}',
            name: "HORSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F40F}',
            name: "RAM".to_string(),
        },
        Emoji {
            unicode: '\u{1F410}',
            name: "GOAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F411}',
            name: "SHEEP".to_string(),
        },
        Emoji {
            unicode: '\u{1F412}',
            name: "MONKEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F413}',
            name: "ROOSTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F414}',
            name: "CHICKEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F415}',
            name: "DOG".to_string(),
        },
        Emoji {
            unicode: '\u{1F416}',
            name: "PIG".to_string(),
        },
        Emoji {
            unicode: '\u{1F417}',
            name: "BOAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F418}',
            name: "ELEPHANT".to_string(),
        },
        Emoji {
            unicode: '\u{1F419}',
            name: "OCTOPUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F41A}',
            name: "SPIRAL SHELL".to_string(),
        },
        Emoji {
            unicode: '\u{1F41B}',
            name: "BUG".to_string(),
        },
        Emoji {
            unicode: '\u{1F41C}',
            name: "ANT".to_string(),
        },
        Emoji {
            unicode: '\u{1F41D}',
            name: "HONEYBEE".to_string(),
        },
        Emoji {
            unicode: '\u{1F41E}',
            name: "LADY BEETLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F41F}',
            name: "FISH".to_string(),
        },
        Emoji {
            unicode: '\u{1F420}',
            name: "TROPICAL FISH".to_string(),
        },
        Emoji {
            unicode: '\u{1F421}',
            name: "BLOWFISH".to_string(),
        },
        Emoji {
            unicode: '\u{1F422}',
            name: "TURTLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F423}',
            name: "HATCHING CHICK".to_string(),
        },
        Emoji {
            unicode: '\u{1F424}',
            name: "BABY CHICK".to_string(),
        },
        Emoji {
            unicode: '\u{1F425}',
            name: "FRONT-FACING BABY CHICK".to_string(),
        },
        Emoji {
            unicode: '\u{1F426}',
            name: "BIRD".to_string(),
        },
        Emoji {
            unicode: '\u{1F427}',
            name: "PENGUIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F428}',
            name: "KOALA".to_string(),
        },
        Emoji {
            unicode: '\u{1F429}',
            name: "POODLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F42A}',
            name: "DROMEDARY CAMEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F42B}',
            name: "BACTRIAN CAMEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F42C}',
            name: "DOLPHIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F42D}',
            name: "MOUSE FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F42E}',
            name: "COW FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F42F}',
            name: "TIGER FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F430}',
            name: "RABBIT FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F431}',
            name: "CAT FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F432}',
            name: "DRAGON FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F433}',
            name: "SPOUTING WHALE".to_string(),
        },
        Emoji {
            unicode: '\u{1F434}',
            name: "HORSE FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F435}',
            name: "MONKEY FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F436}',
            name: "DOG FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F437}',
            name: "PIG FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F438}',
            name: "FROG FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F439}',
            name: "HAMSTER FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F43A}',
            name: "WOLF FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F43B}',
            name: "BEAR FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F43C}',
            name: "PANDA FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F43D}',
            name: "PIG NOSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F43E}',
            name: "PAW PRINTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F43F}',
            name: "CHIPMUNK".to_string(),
        },
        Emoji {
            unicode: '\u{1F440}',
            name: "EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F441}',
            name: "EYE".to_string(),
        },
        Emoji {
            unicode: '\u{1F442}',
            name: "EAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F443}',
            name: "NOSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F444}',
            name: "MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F445}',
            name: "TONGUE".to_string(),
        },
        Emoji {
            unicode: '\u{1F446}',
            name: "WHITE UP POINTING BACKHAND INDEX".to_string(),
        },
        Emoji {
            unicode: '\u{1F447}',
            name: "WHITE DOWN POINTING BACKHAND INDEX".to_string(),
        },
        Emoji {
            unicode: '\u{1F448}',
            name: "WHITE LEFT POINTING BACKHAND INDEX".to_string(),
        },
        Emoji {
            unicode: '\u{1F449}',
            name: "WHITE RIGHT POINTING BACKHAND INDEX".to_string(),
        },
        Emoji {
            unicode: '\u{1F44A}',
            name: "FISTED HAND SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F44B}',
            name: "WAVING HAND SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F44C}',
            name: "OK HAND SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F44D}',
            name: "THUMBS UP SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F44E}',
            name: "THUMBS DOWN SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F44F}',
            name: "CLAPPING HANDS SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F450}',
            name: "OPEN HANDS SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F451}',
            name: "CROWN".to_string(),
        },
        Emoji {
            unicode: '\u{1F452}',
            name: "WOMANS HAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F453}',
            name: "EYEGLASSES".to_string(),
        },
        Emoji {
            unicode: '\u{1F454}',
            name: "NECKTIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F455}',
            name: "T-SHIRT".to_string(),
        },
        Emoji {
            unicode: '\u{1F456}',
            name: "JEANS".to_string(),
        },
        Emoji {
            unicode: '\u{1F457}',
            name: "DRESS".to_string(),
        },
        Emoji {
            unicode: '\u{1F458}',
            name: "KIMONO".to_string(),
        },
        Emoji {
            unicode: '\u{1F459}',
            name: "BIKINI".to_string(),
        },
        Emoji {
            unicode: '\u{1F45A}',
            name: "WOMANS CLOTHES".to_string(),
        },
        Emoji {
            unicode: '\u{1F45B}',
            name: "PURSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F45C}',
            name: "HANDBAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F45D}',
            name: "POUCH".to_string(),
        },
        Emoji {
            unicode: '\u{1F45E}',
            name: "MANS SHOE".to_string(),
        },
        Emoji {
            unicode: '\u{1F45F}',
            name: "ATHLETIC SHOE".to_string(),
        },
        Emoji {
            unicode: '\u{1F460}',
            name: "HIGH-HEELED SHOE".to_string(),
        },
        Emoji {
            unicode: '\u{1F461}',
            name: "WOMANS SANDAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F462}',
            name: "WOMANS BOOTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F463}',
            name: "FOOTPRINTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F464}',
            name: "BUST IN SILHOUETTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F465}',
            name: "BUSTS IN SILHOUETTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F466}',
            name: "BOY".to_string(),
        },
        Emoji {
            unicode: '\u{1F467}',
            name: "GIRL".to_string(),
        },
        Emoji {
            unicode: '\u{1F468}',
            name: "MAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F469}',
            name: "WOMAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F46A}',
            name: "FAMILY".to_string(),
        },
        Emoji {
            unicode: '\u{1F46B}',
            name: "MAN AND WOMAN HOLDING HANDS".to_string(),
        },
        Emoji {
            unicode: '\u{1F46C}',
            name: "TWO MEN HOLDING HANDS".to_string(),
        },
        Emoji {
            unicode: '\u{1F46D}',
            name: "TWO WOMEN HOLDING HANDS".to_string(),
        },
        Emoji {
            unicode: '\u{1F46E}',
            name: "POLICE OFFICER".to_string(),
        },
        Emoji {
            unicode: '\u{1F46F}',
            name: "WOMAN WITH BUNNY EARS".to_string(),
        },
        Emoji {
            unicode: '\u{1F470}',
            name: "BRIDE WITH VEIL".to_string(),
        },
        Emoji {
            unicode: '\u{1F471}',
            name: "PERSON WITH BLOND HAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1F472}',
            name: "MAN WITH GUA PI MAO".to_string(),
        },
        Emoji {
            unicode: '\u{1F473}',
            name: "MAN WITH TURBAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F474}',
            name: "OLDER MAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F475}',
            name: "OLDER WOMAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F476}',
            name: "BABY".to_string(),
        },
        Emoji {
            unicode: '\u{1F477}',
            name: "CONSTRUCTION WORKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F478}',
            name: "PRINCESS".to_string(),
        },
        Emoji {
            unicode: '\u{1F479}',
            name: "JAPANESE OGRE".to_string(),
        },
        Emoji {
            unicode: '\u{1F47A}',
            name: "JAPANESE GOBLIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F47B}',
            name: "GHOST".to_string(),
        },
        Emoji {
            unicode: '\u{1F47C}',
            name: "BABY ANGEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F47D}',
            name: "EXTRATERRESTRIAL ALIEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F47E}',
            name: "ALIEN MONSTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F47F}',
            name: "IMP".to_string(),
        },
        Emoji {
            unicode: '\u{1F480}',
            name: "SKULL".to_string(),
        },
        Emoji {
            unicode: '\u{1F481}',
            name: "INFORMATION DESK PERSON".to_string(),
        },
        Emoji {
            unicode: '\u{1F482}',
            name: "GUARDSMAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F483}',
            name: "DANCER".to_string(),
        },
        Emoji {
            unicode: '\u{1F484}',
            name: "LIPSTICK".to_string(),
        },
        Emoji {
            unicode: '\u{1F485}',
            name: "NAIL POLISH".to_string(),
        },
        Emoji {
            unicode: '\u{1F486}',
            name: "FACE MASSAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F487}',
            name: "HAIRCUT".to_string(),
        },
        Emoji {
            unicode: '\u{1F488}',
            name: "BARBER POLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F489}',
            name: "SYRINGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F48A}',
            name: "PILL".to_string(),
        },
        Emoji {
            unicode: '\u{1F48B}',
            name: "KISS MARK".to_string(),
        },
        Emoji {
            unicode: '\u{1F48C}',
            name: "LOVE LETTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F48D}',
            name: "RING".to_string(),
        },
        Emoji {
            unicode: '\u{1F48E}',
            name: "GEM STONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F48F}',
            name: "KISS".to_string(),
        },
        Emoji {
            unicode: '\u{1F490}',
            name: "BOUQUET".to_string(),
        },
        Emoji {
            unicode: '\u{1F491}',
            name: "COUPLE WITH HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F492}',
            name: "WEDDING".to_string(),
        },
        Emoji {
            unicode: '\u{1F493}',
            name: "BEATING HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F494}',
            name: "BROKEN HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F495}',
            name: "TWO HEARTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F496}',
            name: "SPARKLING HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F497}',
            name: "GROWING HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F498}',
            name: "HEART WITH ARROW".to_string(),
        },
        Emoji {
            unicode: '\u{1F499}',
            name: "BLUE HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F49A}',
            name: "GREEN HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F49B}',
            name: "YELLOW HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F49C}',
            name: "PURPLE HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F49D}',
            name: "HEART WITH RIBBON".to_string(),
        },
        Emoji {
            unicode: '\u{1F49E}',
            name: "REVOLVING HEARTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F49F}',
            name: "HEART DECORATION".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A0}',
            name: "DIAMOND SHAPE WITH A DOT INSIDE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A1}',
            name: "ELECTRIC LIGHT BULB".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A2}',
            name: "ANGER SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A3}',
            name: "BOMB".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A4}',
            name: "SLEEPING SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A5}',
            name: "COLLISION SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A6}',
            name: "SPLASHING SWEAT SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A7}',
            name: "DROPLET".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A8}',
            name: "DASH SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4A9}',
            name: "PILE OF POO".to_string(),
        },
        Emoji {
            unicode: '\u{1F4AA}',
            name: "FLEXED BICEPS".to_string(),
        },
        Emoji {
            unicode: '\u{1F4AB}',
            name: "DIZZY SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4AC}',
            name: "SPEECH BALLOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F4AD}',
            name: "THOUGHT BALLOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F4AE}',
            name: "WHITE FLOWER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4AF}',
            name: "HUNDRED POINTS SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B0}',
            name: "MONEY BAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B1}',
            name: "CURRENCY EXCHANGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B2}',
            name: "HEAVY DOLLAR SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B3}',
            name: "CREDIT CARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B4}',
            name: "BANKNOTE WITH YEN SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B5}',
            name: "BANKNOTE WITH DOLLAR SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B6}',
            name: "BANKNOTE WITH EURO SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B7}',
            name: "BANKNOTE WITH POUND SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B8}',
            name: "MONEY WITH WINGS".to_string(),
        },
        Emoji {
            unicode: '\u{1F4B9}',
            name: "CHART WITH UPWARDS TREND AND YEN SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4BA}',
            name: "SEAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F4BB}',
            name: "PERSONAL COMPUTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4BC}',
            name: "BRIEFCASE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4BD}',
            name: "MINIDISC".to_string(),
        },
        Emoji {
            unicode: '\u{1F4BE}',
            name: "FLOPPY DISK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4BF}',
            name: "OPTICAL DISC".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C0}',
            name: "DVD".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C1}',
            name: "FILE FOLDER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C2}',
            name: "OPEN FILE FOLDER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C3}',
            name: "PAGE WITH CURL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C4}',
            name: "PAGE FACING UP".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C5}',
            name: "CALENDAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C6}',
            name: "TEAR-OFF CALENDAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C7}',
            name: "CARD INDEX".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C8}',
            name: "CHART WITH UPWARDS TREND".to_string(),
        },
        Emoji {
            unicode: '\u{1F4C9}',
            name: "CHART WITH DOWNWARDS TREND".to_string(),
        },
        Emoji {
            unicode: '\u{1F4CA}',
            name: "BAR CHART".to_string(),
        },
        Emoji {
            unicode: '\u{1F4CB}',
            name: "CLIPBOARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F4CC}',
            name: "PUSHPIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4CD}',
            name: "ROUND PUSHPIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4CE}',
            name: "PAPERCLIP".to_string(),
        },
        Emoji {
            unicode: '\u{1F4CF}',
            name: "STRAIGHT RULER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D0}',
            name: "TRIANGULAR RULER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D1}',
            name: "BOOKMARK TABS".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D2}',
            name: "LEDGER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D3}',
            name: "NOTEBOOK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D4}',
            name: "NOTEBOOK WITH DECORATIVE COVER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D5}',
            name: "CLOSED BOOK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D6}',
            name: "OPEN BOOK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D7}',
            name: "GREEN BOOK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D8}',
            name: "BLUE BOOK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4D9}',
            name: "ORANGE BOOK".to_string(),
        },
        Emoji {
            unicode: '\u{1F4DA}',
            name: "BOOKS".to_string(),
        },
        Emoji {
            unicode: '\u{1F4DB}',
            name: "NAME BADGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4DC}',
            name: "SCROLL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4DD}',
            name: "MEMO".to_string(),
        },
        Emoji {
            unicode: '\u{1F4DE}',
            name: "TELEPHONE RECEIVER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4DF}',
            name: "PAGER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E0}',
            name: "FAX MACHINE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E1}',
            name: "SATELLITE ANTENNA".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E2}',
            name: "PUBLIC ADDRESS LOUDSPEAKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E3}',
            name: "CHEERING MEGAPHONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E4}',
            name: "OUTBOX TRAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E5}',
            name: "INBOX TRAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E6}',
            name: "PACKAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E7}',
            name: "E-MAIL SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E8}',
            name: "INCOMING ENVELOPE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4E9}',
            name: "ENVELOPE WITH DOWNWARDS ARROW ABOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4EA}',
            name: "CLOSED MAILBOX WITH LOWERED FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F4EB}',
            name: "CLOSED MAILBOX WITH RAISED FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F4EC}',
            name: "OPEN MAILBOX WITH RAISED FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F4ED}',
            name: "OPEN MAILBOX WITH LOWERED FLAG".to_string(),
        },
        Emoji {
            unicode: '\u{1F4EE}',
            name: "POSTBOX".to_string(),
        },
        Emoji {
            unicode: '\u{1F4EF}',
            name: "POSTAL HORN".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F0}',
            name: "NEWSPAPER".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F1}',
            name: "MOBILE PHONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F2}',
            name: "MOBILE PHONE WITH RIGHTWARDS ARROW AT LEFT".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F3}',
            name: "VIBRATION MODE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F4}',
            name: "MOBILE PHONE OFF".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F5}',
            name: "NO MOBILE PHONES".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F6}',
            name: "ANTENNA WITH BARS".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F7}',
            name: "CAMERA".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F8}',
            name: "CAMERA WITH FLASH".to_string(),
        },
        Emoji {
            unicode: '\u{1F4F9}',
            name: "VIDEO CAMERA".to_string(),
        },
        Emoji {
            unicode: '\u{1F4FA}',
            name: "TELEVISION".to_string(),
        },
        Emoji {
            unicode: '\u{1F4FB}',
            name: "RADIO".to_string(),
        },
        Emoji {
            unicode: '\u{1F4FC}',
            name: "VIDEOCASSETTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F4FD}',
            name: "FILM PROJECTOR".to_string(),
        },
        Emoji {
            unicode: '\u{1F4FF}',
            name: "PRAYER BEADS".to_string(),
        },
        Emoji {
            unicode: '\u{1F500}',
            name: "TWISTED RIGHTWARDS ARROWS".to_string(),
        },
        Emoji {
            unicode: '\u{1F501}',
            name: "CLOCKWISE RIGHTWARDS AND LEFTWARDS OPEN CIRCLE ARROWS".to_string(),
        },
        Emoji {
            unicode: '\u{1F502}',
            name: "CLOCKWISE RIGHTWARDS AND LEFTWARDS OPEN CIRCLE ARROWS WITH CIRCLED ONE OVERLAY"
                .to_string(),
        },
        Emoji {
            unicode: '\u{1F503}',
            name: "CLOCKWISE DOWNWARDS AND UPWARDS OPEN CIRCLE ARROWS".to_string(),
        },
        Emoji {
            unicode: '\u{1F504}',
            name: "ANTICLOCKWISE DOWNWARDS AND UPWARDS OPEN CIRCLE ARROWS".to_string(),
        },
        Emoji {
            unicode: '\u{1F505}',
            name: "LOW BRIGHTNESS SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F506}',
            name: "HIGH BRIGHTNESS SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F507}',
            name: "SPEAKER WITH CANCELLATION STROKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F508}',
            name: "SPEAKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F509}',
            name: "SPEAKER WITH ONE SOUND WAVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F50A}',
            name: "SPEAKER WITH THREE SOUND WAVES".to_string(),
        },
        Emoji {
            unicode: '\u{1F50B}',
            name: "BATTERY".to_string(),
        },
        Emoji {
            unicode: '\u{1F50C}',
            name: "ELECTRIC PLUG".to_string(),
        },
        Emoji {
            unicode: '\u{1F50D}',
            name: "LEFT-POINTING MAGNIFYING GLASS".to_string(),
        },
        Emoji {
            unicode: '\u{1F50E}',
            name: "RIGHT-POINTING MAGNIFYING GLASS".to_string(),
        },
        Emoji {
            unicode: '\u{1F50F}',
            name: "LOCK WITH INK PEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F510}',
            name: "CLOSED LOCK WITH KEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F511}',
            name: "KEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F512}',
            name: "LOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F513}',
            name: "OPEN LOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F514}',
            name: "BELL".to_string(),
        },
        Emoji {
            unicode: '\u{1F515}',
            name: "BELL WITH CANCELLATION STROKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F516}',
            name: "BOOKMARK".to_string(),
        },
        Emoji {
            unicode: '\u{1F517}',
            name: "LINK SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F518}',
            name: "RADIO BUTTON".to_string(),
        },
        Emoji {
            unicode: '\u{1F519}',
            name: "BACK WITH LEFTWARDS ARROW ABOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F51A}',
            name: "END WITH LEFTWARDS ARROW ABOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F51B}',
            name: "ON WITH EXCLAMATION MARK WITH LEFT RIGHT ARROW ABOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F51C}',
            name: "SOON WITH RIGHTWARDS ARROW ABOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F51D}',
            name: "TOP WITH UPWARDS ARROW ABOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F51E}',
            name: "NO ONE UNDER EIGHTEEN SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F51F}',
            name: "KEYCAP TEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F520}',
            name: "INPUT SYMBOL FOR LATIN CAPITAL LETTERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F521}',
            name: "INPUT SYMBOL FOR LATIN SMALL LETTERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F522}',
            name: "INPUT SYMBOL FOR NUMBERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F523}',
            name: "INPUT SYMBOL FOR SYMBOLS".to_string(),
        },
        Emoji {
            unicode: '\u{1F524}',
            name: "INPUT SYMBOL FOR LATIN LETTERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F525}',
            name: "FIRE".to_string(),
        },
        Emoji {
            unicode: '\u{1F526}',
            name: "ELECTRIC TORCH".to_string(),
        },
        Emoji {
            unicode: '\u{1F527}',
            name: "WRENCH".to_string(),
        },
        Emoji {
            unicode: '\u{1F528}',
            name: "HAMMER".to_string(),
        },
        Emoji {
            unicode: '\u{1F529}',
            name: "NUT AND BOLT".to_string(),
        },
        Emoji {
            unicode: '\u{1F52A}',
            name: "HOCHO".to_string(),
        },
        Emoji {
            unicode: '\u{1F52B}',
            name: "PISTOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F52C}',
            name: "MICROSCOPE".to_string(),
        },
        Emoji {
            unicode: '\u{1F52D}',
            name: "TELESCOPE".to_string(),
        },
        Emoji {
            unicode: '\u{1F52E}',
            name: "CRYSTAL BALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F52F}',
            name: "SIX POINTED STAR WITH MIDDLE DOT".to_string(),
        },
        Emoji {
            unicode: '\u{1F530}',
            name: "JAPANESE SYMBOL FOR BEGINNER".to_string(),
        },
        Emoji {
            unicode: '\u{1F531}',
            name: "TRIDENT EMBLEM".to_string(),
        },
        Emoji {
            unicode: '\u{1F532}',
            name: "BLACK SQUARE BUTTON".to_string(),
        },
        Emoji {
            unicode: '\u{1F533}',
            name: "WHITE SQUARE BUTTON".to_string(),
        },
        Emoji {
            unicode: '\u{1F534}',
            name: "LARGE RED CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F535}',
            name: "LARGE BLUE CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F536}',
            name: "LARGE ORANGE DIAMOND".to_string(),
        },
        Emoji {
            unicode: '\u{1F537}',
            name: "LARGE BLUE DIAMOND".to_string(),
        },
        Emoji {
            unicode: '\u{1F538}',
            name: "SMALL ORANGE DIAMOND".to_string(),
        },
        Emoji {
            unicode: '\u{1F539}',
            name: "SMALL BLUE DIAMOND".to_string(),
        },
        Emoji {
            unicode: '\u{1F53A}',
            name: "UP-POINTING RED TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F53B}',
            name: "DOWN-POINTING RED TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F53C}',
            name: "UP-POINTING SMALL RED TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F53D}',
            name: "DOWN-POINTING SMALL RED TRIANGLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F549}',
            name: "OM SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F54A}',
            name: "DOVE OF PEACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F54B}',
            name: "KAABA".to_string(),
        },
        Emoji {
            unicode: '\u{1F54C}',
            name: "MOSQUE".to_string(),
        },
        Emoji {
            unicode: '\u{1F54D}',
            name: "SYNAGOGUE".to_string(),
        },
        Emoji {
            unicode: '\u{1F54E}',
            name: "MENORAH WITH NINE BRANCHES".to_string(),
        },
        Emoji {
            unicode: '\u{1F550}',
            name: "CLOCK FACE ONE OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F551}',
            name: "CLOCK FACE TWO OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F552}',
            name: "CLOCK FACE THREE OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F553}',
            name: "CLOCK FACE FOUR OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F554}',
            name: "CLOCK FACE FIVE OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F555}',
            name: "CLOCK FACE SIX OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F556}',
            name: "CLOCK FACE SEVEN OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F557}',
            name: "CLOCK FACE EIGHT OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F558}',
            name: "CLOCK FACE NINE OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F559}',
            name: "CLOCK FACE TEN OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F55A}',
            name: "CLOCK FACE ELEVEN OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F55B}',
            name: "CLOCK FACE TWELVE OCLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F55C}',
            name: "CLOCK FACE ONE-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F55D}',
            name: "CLOCK FACE TWO-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F55E}',
            name: "CLOCK FACE THREE-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F55F}',
            name: "CLOCK FACE FOUR-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F560}',
            name: "CLOCK FACE FIVE-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F561}',
            name: "CLOCK FACE SIX-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F562}',
            name: "CLOCK FACE SEVEN-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F563}',
            name: "CLOCK FACE EIGHT-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F564}',
            name: "CLOCK FACE NINE-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F565}',
            name: "CLOCK FACE TEN-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F566}',
            name: "CLOCK FACE ELEVEN-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F567}',
            name: "CLOCK FACE TWELVE-THIRTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F56F}',
            name: "CANDLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F570}',
            name: "MANTELPIECE CLOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F573}',
            name: "HOLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F574}',
            name: "MAN IN BUSINESS SUIT LEVITATING".to_string(),
        },
        Emoji {
            unicode: '\u{1F575}',
            name: "SLEUTH OR SPY".to_string(),
        },
        Emoji {
            unicode: '\u{1F576}',
            name: "DARK SUNGLASSES".to_string(),
        },
        Emoji {
            unicode: '\u{1F577}',
            name: "SPIDER".to_string(),
        },
        Emoji {
            unicode: '\u{1F578}',
            name: "SPIDER WEB".to_string(),
        },
        Emoji {
            unicode: '\u{1F579}',
            name: "JOYSTICK".to_string(),
        },
        Emoji {
            unicode: '\u{1F57A}',
            name: "MAN DANCING".to_string(),
        },
        Emoji {
            unicode: '\u{1F587}',
            name: "LINKED PAPERCLIPS".to_string(),
        },
        Emoji {
            unicode: '\u{1F58A}',
            name: "LOWER LEFT BALLPOINT PEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F58B}',
            name: "LOWER LEFT FOUNTAIN PEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F58C}',
            name: "LOWER LEFT PAINTBRUSH".to_string(),
        },
        Emoji {
            unicode: '\u{1F58D}',
            name: "LOWER LEFT CRAYON".to_string(),
        },
        Emoji {
            unicode: '\u{1F590}',
            name: "RAISED HAND WITH FINGERS SPLAYED".to_string(),
        },
        Emoji {
            unicode: '\u{1F595}',
            name: "REVERSED HAND WITH MIDDLE FINGER EXTENDED".to_string(),
        },
        Emoji {
            unicode: '\u{1F596}',
            name: "RAISED HAND WITH PART BETWEEN MIDDLE AND RING FINGERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F5A4}',
            name: "BLACK HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F5A5}',
            name: "DESKTOP COMPUTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F5A8}',
            name: "PRINTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F5B1}',
            name: "THREE BUTTON MOUSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F5B2}',
            name: "TRACKBALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F5BC}',
            name: "FRAME WITH PICTURE".to_string(),
        },
        Emoji {
            unicode: '\u{1F5C2}',
            name: "CARD INDEX DIVIDERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F5C3}',
            name: "CARD FILE BOX".to_string(),
        },
        Emoji {
            unicode: '\u{1F5C4}',
            name: "FILE CABINET".to_string(),
        },
        Emoji {
            unicode: '\u{1F5D1}',
            name: "WASTEBASKET".to_string(),
        },
        Emoji {
            unicode: '\u{1F5D2}',
            name: "SPIRAL NOTE PAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F5D3}',
            name: "SPIRAL CALENDAR PAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F5DC}',
            name: "COMPRESSION".to_string(),
        },
        Emoji {
            unicode: '\u{1F5DD}',
            name: "OLD KEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F5DE}',
            name: "ROLLED-UP NEWSPAPER".to_string(),
        },
        Emoji {
            unicode: '\u{1F5E1}',
            name: "DAGGER KNIFE".to_string(),
        },
        Emoji {
            unicode: '\u{1F5E3}',
            name: "SPEAKING HEAD IN SILHOUETTE".to_string(),
        },
        Emoji {
            unicode: '\u{1F5E8}',
            name: "LEFT SPEECH BUBBLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F5EF}',
            name: "RIGHT ANGER BUBBLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F5F3}',
            name: "BALLOT BOX WITH BALLOT".to_string(),
        },
        Emoji {
            unicode: '\u{1F5FA}',
            name: "WORLD MAP".to_string(),
        },
        Emoji {
            unicode: '\u{1F5FB}',
            name: "MOUNT FUJI".to_string(),
        },
        Emoji {
            unicode: '\u{1F5FC}',
            name: "TOKYO TOWER".to_string(),
        },
        Emoji {
            unicode: '\u{1F5FD}',
            name: "STATUE OF LIBERTY".to_string(),
        },
        Emoji {
            unicode: '\u{1F5FE}',
            name: "SILHOUETTE OF JAPAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F5FF}',
            name: "MOYAI".to_string(),
        },
        Emoji {
            unicode: '\u{1F600}',
            name: "GRINNING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F601}',
            name: "GRINNING FACE WITH SMILING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F602}',
            name: "FACE WITH TEARS OF JOY".to_string(),
        },
        Emoji {
            unicode: '\u{1F603}',
            name: "SMILING FACE WITH OPEN MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F604}',
            name: "SMILING FACE WITH OPEN MOUTH AND SMILING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F605}',
            name: "SMILING FACE WITH OPEN MOUTH AND COLD SWEAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F606}',
            name: "SMILING FACE WITH OPEN MOUTH AND TIGHTLY-CLOSED EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F607}',
            name: "SMILING FACE WITH HALO".to_string(),
        },
        Emoji {
            unicode: '\u{1F608}',
            name: "SMILING FACE WITH HORNS".to_string(),
        },
        Emoji {
            unicode: '\u{1F609}',
            name: "WINKING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F60A}',
            name: "SMILING FACE WITH SMILING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F60B}',
            name: "FACE SAVOURING DELICIOUS FOOD".to_string(),
        },
        Emoji {
            unicode: '\u{1F60C}',
            name: "RELIEVED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F60D}',
            name: "SMILING FACE WITH HEART-SHAPED EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F60E}',
            name: "SMILING FACE WITH SUNGLASSES".to_string(),
        },
        Emoji {
            unicode: '\u{1F60F}',
            name: "SMIRKING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F610}',
            name: "NEUTRAL FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F611}',
            name: "EXPRESSIONLESS FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F612}',
            name: "UNAMUSED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F613}',
            name: "FACE WITH COLD SWEAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F614}',
            name: "PENSIVE FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F615}',
            name: "CONFUSED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F616}',
            name: "CONFOUNDED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F617}',
            name: "KISSING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F618}',
            name: "FACE THROWING A KISS".to_string(),
        },
        Emoji {
            unicode: '\u{1F619}',
            name: "KISSING FACE WITH SMILING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F61A}',
            name: "KISSING FACE WITH CLOSED EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F61B}',
            name: "FACE WITH STUCK-OUT TONGUE".to_string(),
        },
        Emoji {
            unicode: '\u{1F61C}',
            name: "FACE WITH STUCK-OUT TONGUE AND WINKING EYE".to_string(),
        },
        Emoji {
            unicode: '\u{1F61D}',
            name: "FACE WITH STUCK-OUT TONGUE AND TIGHTLY-CLOSED EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F61E}',
            name: "DISAPPOINTED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F61F}',
            name: "WORRIED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F620}',
            name: "ANGRY FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F621}',
            name: "POUTING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F622}',
            name: "CRYING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F623}',
            name: "PERSEVERING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F624}',
            name: "FACE WITH LOOK OF TRIUMPH".to_string(),
        },
        Emoji {
            unicode: '\u{1F625}',
            name: "DISAPPOINTED BUT RELIEVED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F626}',
            name: "FROWNING FACE WITH OPEN MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F627}',
            name: "ANGUISHED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F628}',
            name: "FEARFUL FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F629}',
            name: "WEARY FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F62A}',
            name: "SLEEPY FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F62B}',
            name: "TIRED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F62C}',
            name: "GRIMACING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F62D}',
            name: "LOUDLY CRYING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F62E}',
            name: "FACE WITH OPEN MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F62F}',
            name: "HUSHED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F630}',
            name: "FACE WITH OPEN MOUTH AND COLD SWEAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F631}',
            name: "FACE SCREAMING IN FEAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F632}',
            name: "ASTONISHED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F633}',
            name: "FLUSHED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F634}',
            name: "SLEEPING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F635}',
            name: "DIZZY FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F636}',
            name: "FACE WITHOUT MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F637}',
            name: "FACE WITH MEDICAL MASK".to_string(),
        },
        Emoji {
            unicode: '\u{1F638}',
            name: "GRINNING CAT FACE WITH SMILING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F639}',
            name: "CAT FACE WITH TEARS OF JOY".to_string(),
        },
        Emoji {
            unicode: '\u{1F63A}',
            name: "SMILING CAT FACE WITH OPEN MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F63B}',
            name: "SMILING CAT FACE WITH HEART-SHAPED EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F63C}',
            name: "CAT FACE WITH WRY SMILE".to_string(),
        },
        Emoji {
            unicode: '\u{1F63D}',
            name: "KISSING CAT FACE WITH CLOSED EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F63E}',
            name: "POUTING CAT FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F63F}',
            name: "CRYING CAT FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F640}',
            name: "WEARY CAT FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F641}',
            name: "SLIGHTLY FROWNING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F642}',
            name: "SLIGHTLY SMILING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F643}',
            name: "UPSIDE-DOWN FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F644}',
            name: "FACE WITH ROLLING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F645}',
            name: "FACE WITH NO GOOD GESTURE".to_string(),
        },
        Emoji {
            unicode: '\u{1F646}',
            name: "FACE WITH OK GESTURE".to_string(),
        },
        Emoji {
            unicode: '\u{1F647}',
            name: "PERSON BOWING DEEPLY".to_string(),
        },
        Emoji {
            unicode: '\u{1F648}',
            name: "SEE-NO-EVIL MONKEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F649}',
            name: "HEAR-NO-EVIL MONKEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F64A}',
            name: "SPEAK-NO-EVIL MONKEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F64B}',
            name: "HAPPY PERSON RAISING ONE HAND".to_string(),
        },
        Emoji {
            unicode: '\u{1F64C}',
            name: "PERSON RAISING BOTH HANDS IN CELEBRATION".to_string(),
        },
        Emoji {
            unicode: '\u{1F64D}',
            name: "PERSON FROWNING".to_string(),
        },
        Emoji {
            unicode: '\u{1F64E}',
            name: "PERSON WITH POUTING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F64F}',
            name: "PERSON WITH FOLDED HANDS".to_string(),
        },
        Emoji {
            unicode: '\u{1F680}',
            name: "ROCKET".to_string(),
        },
        Emoji {
            unicode: '\u{1F681}',
            name: "HELICOPTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F682}',
            name: "STEAM LOCOMOTIVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F683}',
            name: "RAILWAY CAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F684}',
            name: "HIGH-SPEED TRAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F685}',
            name: "HIGH-SPEED TRAIN WITH BULLET NOSE".to_string(),
        },
        Emoji {
            unicode: '\u{1F686}',
            name: "TRAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F687}',
            name: "METRO".to_string(),
        },
        Emoji {
            unicode: '\u{1F688}',
            name: "LIGHT RAIL".to_string(),
        },
        Emoji {
            unicode: '\u{1F689}',
            name: "STATION".to_string(),
        },
        Emoji {
            unicode: '\u{1F68A}',
            name: "TRAM".to_string(),
        },
        Emoji {
            unicode: '\u{1F68B}',
            name: "TRAM CAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F68C}',
            name: "BUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F68D}',
            name: "ONCOMING BUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F68E}',
            name: "TROLLEYBUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F68F}',
            name: "BUS STOP".to_string(),
        },
        Emoji {
            unicode: '\u{1F690}',
            name: "MINIBUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F691}',
            name: "AMBULANCE".to_string(),
        },
        Emoji {
            unicode: '\u{1F692}',
            name: "FIRE ENGINE".to_string(),
        },
        Emoji {
            unicode: '\u{1F693}',
            name: "POLICE CAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F694}',
            name: "ONCOMING POLICE CAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F695}',
            name: "TAXI".to_string(),
        },
        Emoji {
            unicode: '\u{1F696}',
            name: "ONCOMING TAXI".to_string(),
        },
        Emoji {
            unicode: '\u{1F697}',
            name: "AUTOMOBILE".to_string(),
        },
        Emoji {
            unicode: '\u{1F698}',
            name: "ONCOMING AUTOMOBILE".to_string(),
        },
        Emoji {
            unicode: '\u{1F699}',
            name: "RECREATIONAL VEHICLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F69A}',
            name: "DELIVERY TRUCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F69B}',
            name: "ARTICULATED LORRY".to_string(),
        },
        Emoji {
            unicode: '\u{1F69C}',
            name: "TRACTOR".to_string(),
        },
        Emoji {
            unicode: '\u{1F69D}',
            name: "MONORAIL".to_string(),
        },
        Emoji {
            unicode: '\u{1F69E}',
            name: "MOUNTAIN RAILWAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F69F}',
            name: "SUSPENSION RAILWAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A0}',
            name: "MOUNTAIN CABLEWAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A1}',
            name: "AERIAL TRAMWAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A2}',
            name: "SHIP".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A3}',
            name: "ROWBOAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A4}',
            name: "SPEEDBOAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A5}',
            name: "HORIZONTAL TRAFFIC LIGHT".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A6}',
            name: "VERTICAL TRAFFIC LIGHT".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A7}',
            name: "CONSTRUCTION SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A8}',
            name: "POLICE CARS REVOLVING LIGHT".to_string(),
        },
        Emoji {
            unicode: '\u{1F6A9}',
            name: "TRIANGULAR FLAG ON POST".to_string(),
        },
        Emoji {
            unicode: '\u{1F6AA}',
            name: "DOOR".to_string(),
        },
        Emoji {
            unicode: '\u{1F6AB}',
            name: "NO ENTRY SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F6AC}',
            name: "SMOKING SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6AD}',
            name: "NO SMOKING SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6AE}',
            name: "PUT LITTER IN ITS PLACE SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6AF}',
            name: "DO NOT LITTER SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B0}',
            name: "POTABLE WATER SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B1}',
            name: "NON-POTABLE WATER SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B2}',
            name: "BICYCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B3}',
            name: "NO BICYCLES".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B4}',
            name: "BICYCLIST".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B5}',
            name: "MOUNTAIN BICYCLIST".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B6}',
            name: "PEDESTRIAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B7}',
            name: "NO PEDESTRIANS".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B8}',
            name: "CHILDREN CROSSING".to_string(),
        },
        Emoji {
            unicode: '\u{1F6B9}',
            name: "MENS SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6BA}',
            name: "WOMENS SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6BB}',
            name: "RESTROOM".to_string(),
        },
        Emoji {
            unicode: '\u{1F6BC}',
            name: "BABY SYMBOL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6BD}',
            name: "TOILET".to_string(),
        },
        Emoji {
            unicode: '\u{1F6BE}',
            name: "WATER CLOSET".to_string(),
        },
        Emoji {
            unicode: '\u{1F6BF}',
            name: "SHOWER".to_string(),
        },
        Emoji {
            unicode: '\u{1F6C0}',
            name: "BATH".to_string(),
        },
        Emoji {
            unicode: '\u{1F6C1}',
            name: "BATHTUB".to_string(),
        },
        Emoji {
            unicode: '\u{1F6C2}',
            name: "PASSPORT CONTROL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6C3}',
            name: "CUSTOMS".to_string(),
        },
        Emoji {
            unicode: '\u{1F6C4}',
            name: "BAGGAGE CLAIM".to_string(),
        },
        Emoji {
            unicode: '\u{1F6C5}',
            name: "LEFT LUGGAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6CB}',
            name: "COUCH AND LAMP".to_string(),
        },
        Emoji {
            unicode: '\u{1F6CC}',
            name: "SLEEPING ACCOMMODATION".to_string(),
        },
        Emoji {
            unicode: '\u{1F6CD}',
            name: "SHOPPING BAGS".to_string(),
        },
        Emoji {
            unicode: '\u{1F6CE}',
            name: "BELLHOP BELL".to_string(),
        },
        Emoji {
            unicode: '\u{1F6CF}',
            name: "BED".to_string(),
        },
        Emoji {
            unicode: '\u{1F6D0}',
            name: "PLACE OF WORSHIP".to_string(),
        },
        Emoji {
            unicode: '\u{1F6D1}',
            name: "OCTAGONAL SIGN".to_string(),
        },
        Emoji {
            unicode: '\u{1F6D2}',
            name: "SHOPPING TROLLEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F6D5}',
            name: "HINDU TEMPLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E0}',
            name: "HAMMER AND WRENCH".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E1}',
            name: "SHIELD".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E2}',
            name: "OIL DRUM".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E3}',
            name: "MOTORWAY".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E4}',
            name: "RAILWAY TRACK".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E5}',
            name: "MOTOR BOAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F6E9}',
            name: "SMALL AIRPLANE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6EB}',
            name: "AIRPLANE DEPARTURE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6EC}',
            name: "AIRPLANE ARRIVING".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F0}',
            name: "SATELLITE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F3}',
            name: "PASSENGER SHIP".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F4}',
            name: "SCOOTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F5}',
            name: "MOTOR SCOOTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F6}',
            name: "CANOE".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F7}',
            name: "SLED".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F8}',
            name: "FLYING SAUCER".to_string(),
        },
        Emoji {
            unicode: '\u{1F6F9}',
            name: "SKATEBOARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F6FA}',
            name: "AUTO RICKSHAW".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E0}',
            name: "LARGE ORANGE CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E1}',
            name: "LARGE YELLOW CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E2}',
            name: "LARGE GREEN CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E3}',
            name: "LARGE PURPLE CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E4}',
            name: "LARGE BROWN CIRCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E5}',
            name: "LARGE RED SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E6}',
            name: "LARGE BLUE SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E7}',
            name: "LARGE ORANGE SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E8}',
            name: "LARGE YELLOW SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7E9}',
            name: "LARGE GREEN SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7EA}',
            name: "LARGE PURPLE SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F7EB}',
            name: "LARGE BROWN SQUARE".to_string(),
        },
        Emoji {
            unicode: '\u{1F90D}',
            name: "WHITE HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F90E}',
            name: "BROWN HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F90F}',
            name: "PINCHING HAND".to_string(),
        },
        Emoji {
            unicode: '\u{1F910}',
            name: "ZIPPER-MOUTH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F911}',
            name: "MONEY-MOUTH FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F912}',
            name: "FACE WITH THERMOMETER".to_string(),
        },
        Emoji {
            unicode: '\u{1F913}',
            name: "NERD FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F914}',
            name: "THINKING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F915}',
            name: "FACE WITH HEAD-BANDAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F916}',
            name: "ROBOT FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F917}',
            name: "HUGGING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F918}',
            name: "SIGN OF THE HORNS".to_string(),
        },
        Emoji {
            unicode: '\u{1F919}',
            name: "CALL ME HAND".to_string(),
        },
        Emoji {
            unicode: '\u{1F91A}',
            name: "RAISED BACK OF HAND".to_string(),
        },
        Emoji {
            unicode: '\u{1F91B}',
            name: "LEFT-FACING FIST".to_string(),
        },
        Emoji {
            unicode: '\u{1F91C}',
            name: "RIGHT-FACING FIST".to_string(),
        },
        Emoji {
            unicode: '\u{1F91D}',
            name: "HANDSHAKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F91E}',
            name: "HAND WITH INDEX AND MIDDLE FINGERS CROSSED".to_string(),
        },
        Emoji {
            unicode: '\u{1F920}',
            name: "FACE WITH COWBOY HAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F921}',
            name: "CLOWN FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F922}',
            name: "NAUSEATED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F923}',
            name: "ROLLING ON THE FLOOR LAUGHING".to_string(),
        },
        Emoji {
            unicode: '\u{1F924}',
            name: "DROOLING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F925}',
            name: "LYING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F926}',
            name: "FACE PALM".to_string(),
        },
        Emoji {
            unicode: '\u{1F927}',
            name: "SNEEZING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F930}',
            name: "PREGNANT WOMAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F933}',
            name: "SELFIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F934}',
            name: "PRINCE".to_string(),
        },
        Emoji {
            unicode: '\u{1F935}',
            name: "MAN IN TUXEDO".to_string(),
        },
        Emoji {
            unicode: '\u{1F936}',
            name: "MOTHER CHRISTMAS".to_string(),
        },
        Emoji {
            unicode: '\u{1F937}',
            name: "SHRUG".to_string(),
        },
        Emoji {
            unicode: '\u{1F938}',
            name: "PERSON DOING CARTWHEEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F939}',
            name: "JUGGLING".to_string(),
        },
        Emoji {
            unicode: '\u{1F93A}',
            name: "FENCER".to_string(),
        },
        Emoji {
            unicode: '\u{1F93C}',
            name: "WRESTLERS".to_string(),
        },
        Emoji {
            unicode: '\u{1F93D}',
            name: "WATER POLO".to_string(),
        },
        Emoji {
            unicode: '\u{1F93E}',
            name: "HANDBALL".to_string(),
        },
        Emoji {
            unicode: '\u{1F93F}',
            name: "DIVING MASK".to_string(),
        },
        Emoji {
            unicode: '\u{1F940}',
            name: "WILTED FLOWER".to_string(),
        },
        Emoji {
            unicode: '\u{1F941}',
            name: "DRUM WITH DRUMSTICKS".to_string(),
        },
        Emoji {
            unicode: '\u{1F942}',
            name: "CLINKING GLASSES".to_string(),
        },
        Emoji {
            unicode: '\u{1F943}',
            name: "TUMBLER GLASS".to_string(),
        },
        Emoji {
            unicode: '\u{1F944}',
            name: "SPOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F945}',
            name: "GOAL NET".to_string(),
        },
        Emoji {
            unicode: '\u{1F947}',
            name: "FIRST PLACE MEDAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F948}',
            name: "SECOND PLACE MEDAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F949}',
            name: "THIRD PLACE MEDAL".to_string(),
        },
        Emoji {
            unicode: '\u{1F94A}',
            name: "BOXING GLOVE".to_string(),
        },
        Emoji {
            unicode: '\u{1F94B}',
            name: "MARTIAL ARTS UNIFORM".to_string(),
        },
        Emoji {
            unicode: '\u{1F950}',
            name: "CROISSANT".to_string(),
        },
        Emoji {
            unicode: '\u{1F951}',
            name: "AVOCADO".to_string(),
        },
        Emoji {
            unicode: '\u{1F952}',
            name: "CUCUMBER".to_string(),
        },
        Emoji {
            unicode: '\u{1F953}',
            name: "BACON".to_string(),
        },
        Emoji {
            unicode: '\u{1F954}',
            name: "POTATO".to_string(),
        },
        Emoji {
            unicode: '\u{1F955}',
            name: "CARROT".to_string(),
        },
        Emoji {
            unicode: '\u{1F956}',
            name: "BAGUETTE BREAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F957}',
            name: "GREEN SALAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F958}',
            name: "SHALLOW PAN OF FOOD".to_string(),
        },
        Emoji {
            unicode: '\u{1F959}',
            name: "STUFFED FLATBREAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F95A}',
            name: "EGG".to_string(),
        },
        Emoji {
            unicode: '\u{1F95B}',
            name: "GLASS OF MILK".to_string(),
        },
        Emoji {
            unicode: '\u{1F95C}',
            name: "PEANUTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F95D}',
            name: "KIWIFRUIT".to_string(),
        },
        Emoji {
            unicode: '\u{1F95E}',
            name: "PANCAKES".to_string(),
        },
        Emoji {
            unicode: '\u{1F95F}',
            name: "DUMPLING".to_string(),
        },
        Emoji {
            unicode: '\u{1F960}',
            name: "FORTUNE COOKIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F961}',
            name: "TAKEOUT BOX".to_string(),
        },
        Emoji {
            unicode: '\u{1F962}',
            name: "CHOPSTICKS".to_string(),
        },
        Emoji {
            unicode: '\u{1F963}',
            name: "BOWL WITH SPOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F964}',
            name: "CUP WITH STRAW".to_string(),
        },
        Emoji {
            unicode: '\u{1F965}',
            name: "COCONUT".to_string(),
        },
        Emoji {
            unicode: '\u{1F966}',
            name: "BROCCOLI".to_string(),
        },
        Emoji {
            unicode: '\u{1F967}',
            name: "PIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F968}',
            name: "PRETZEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F969}',
            name: "CUT OF MEAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F96A}',
            name: "SANDWICH".to_string(),
        },
        Emoji {
            unicode: '\u{1F96B}',
            name: "CANNED FOOD".to_string(),
        },
        Emoji {
            unicode: '\u{1F96C}',
            name: "LEAFY GREEN".to_string(),
        },
        Emoji {
            unicode: '\u{1F96D}',
            name: "MANGO".to_string(),
        },
        Emoji {
            unicode: '\u{1F96E}',
            name: "MOON CAKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F96F}',
            name: "BAGEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F970}',
            name: "SMILING FACE WITH SMILING EYES AND THREE HEARTS".to_string(),
        },
        Emoji {
            unicode: '\u{1F971}',
            name: "YAWNING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F973}',
            name: "FACE WITH PARTY HORN AND PARTY HAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F974}',
            name: "FACE WITH UNEVEN EYES AND WAVY MOUTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F975}',
            name: "OVERHEATED FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F976}',
            name: "FREEZING FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F97A}',
            name: "FACE WITH PLEADING EYES".to_string(),
        },
        Emoji {
            unicode: '\u{1F97B}',
            name: "SARI".to_string(),
        },
        Emoji {
            unicode: '\u{1F97C}',
            name: "LAB COAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F97D}',
            name: "GOGGLESS".to_string(),
        },
        Emoji {
            unicode: '\u{1F97E}',
            name: "HIKING BOOT".to_string(),
        },
        Emoji {
            unicode: '\u{1F97F}',
            name: "FLAT SHOE".to_string(),
        },
        Emoji {
            unicode: '\u{1F980}',
            name: "CRAB".to_string(),
        },
        Emoji {
            unicode: '\u{1F981}',
            name: "LION FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F982}',
            name: "SCORPION".to_string(),
        },
        Emoji {
            unicode: '\u{1F983}',
            name: "TURKEY".to_string(),
        },
        Emoji {
            unicode: '\u{1F984}',
            name: "UNICORN FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F985}',
            name: "EAGLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F986}',
            name: "DUCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F987}',
            name: "BAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F988}',
            name: "SHARK".to_string(),
        },
        Emoji {
            unicode: '\u{1F989}',
            name: "OWL".to_string(),
        },
        Emoji {
            unicode: '\u{1F98A}',
            name: "FOX FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F98B}',
            name: "BUTTERFLY".to_string(),
        },
        Emoji {
            unicode: '\u{1F98C}',
            name: "DEER".to_string(),
        },
        Emoji {
            unicode: '\u{1F98D}',
            name: "GORILLA".to_string(),
        },
        Emoji {
            unicode: '\u{1F98E}',
            name: "LIZARD".to_string(),
        },
        Emoji {
            unicode: '\u{1F98F}',
            name: "RHINOCEROS".to_string(),
        },
        Emoji {
            unicode: '\u{1F990}',
            name: "SHRIMP".to_string(),
        },
        Emoji {
            unicode: '\u{1F991}',
            name: "SQUID".to_string(),
        },
        Emoji {
            unicode: '\u{1F992}',
            name: "GIRAFFE FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F993}',
            name: "ZEBRA FACE".to_string(),
        },
        Emoji {
            unicode: '\u{1F994}',
            name: "HEDGEHOG".to_string(),
        },
        Emoji {
            unicode: '\u{1F995}',
            name: "SAUROPOD".to_string(),
        },
        Emoji {
            unicode: '\u{1F996}',
            name: "T-REX".to_string(),
        },
        Emoji {
            unicode: '\u{1F997}',
            name: "CRICKET".to_string(),
        },
        Emoji {
            unicode: '\u{1F998}',
            name: "KANGAROO".to_string(),
        },
        Emoji {
            unicode: '\u{1F999}',
            name: "LLAMA".to_string(),
        },
        Emoji {
            unicode: '\u{1F99A}',
            name: "PEACOCK".to_string(),
        },
        Emoji {
            unicode: '\u{1F99B}',
            name: "HIPPOPOTAMUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F99C}',
            name: "PARROT".to_string(),
        },
        Emoji {
            unicode: '\u{1F99D}',
            name: "RACCOON".to_string(),
        },
        Emoji {
            unicode: '\u{1F99E}',
            name: "LOBSTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F99F}',
            name: "MOSQUITO".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A0}',
            name: "MICROBE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A1}',
            name: "BADGER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A2}',
            name: "SWAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A5}',
            name: "SLOTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A6}',
            name: "OTTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A7}',
            name: "ORANGUTAN".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A8}',
            name: "SKUNK".to_string(),
        },
        Emoji {
            unicode: '\u{1F9A9}',
            name: "FLAMINGO".to_string(),
        },
        Emoji {
            unicode: '\u{1F9AA}',
            name: "OYSTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C0}',
            name: "CHEESE WEDGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9AE}',
            name: "GUIDE DOG".to_string(),
        },
        Emoji {
            unicode: '\u{1F9AF}',
            name: "PROBING CANE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B0}',
            name: "EMOJI COMPONENT RED HAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B1}',
            name: "EMOJI COMPONENT CURLY HAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B2}',
            name: "EMOJI COMPONENT BALD".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B3}',
            name: "EMOJI COMPONENT WHITE HAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B4}',
            name: "BONE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B5}',
            name: "LEG".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B6}',
            name: "FOOT".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B7}',
            name: "TOOTH".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B8}',
            name: "SUPERHERO".to_string(),
        },
        Emoji {
            unicode: '\u{1F9B9}',
            name: "SUPERVILLAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F9BA}',
            name: "SAFETY VEST".to_string(),
        },
        Emoji {
            unicode: '\u{1F9BB}',
            name: "EAR WITH HEARING AID".to_string(),
        },
        Emoji {
            unicode: '\u{1F9BC}',
            name: "MOTORIZED WHEELCHAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1F9BD}',
            name: "MANUAL WHEELCHAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1F9BE}',
            name: "MECHANICAL ARM".to_string(),
        },
        Emoji {
            unicode: '\u{1F9BF}',
            name: "MECHANICAL LEG".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C0}',
            name: "CHEESE WEDGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C1}',
            name: "CUPCAKE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C2}',
            name: "SALT SHAKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C3}',
            name: "BEVERAGE BOX".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C4}',
            name: "GARLIC".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C5}',
            name: "ONION".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C6}',
            name: "FALAFEL".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C7}',
            name: "WAFFLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C8}',
            name: "BUTTER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9C9}',
            name: "MATE DRINK".to_string(),
        },
        Emoji {
            unicode: '\u{1F9CA}',
            name: "ICE CUBE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9CD}',
            name: "STANDING PERSON".to_string(),
        },
        Emoji {
            unicode: '\u{1F9CE}',
            name: "KNEELING PERSON".to_string(),
        },
        Emoji {
            unicode: '\u{1F9CF}',
            name: "DEAF PERSON".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D0}',
            name: "FACE WITH MONOCLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D1}',
            name: "ADULT".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D2}',
            name: "CHILD".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D3}',
            name: "OLDER ADULT".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D4}',
            name: "BEARDED PERSON".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D5}',
            name: "PERSON WITH HEADSCARF".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D6}',
            name: "PERSON IN STEAMY ROOM".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D7}',
            name: "PERSON CLIMBING".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D8}',
            name: "PERSON IN LOTUS POSITION".to_string(),
        },
        Emoji {
            unicode: '\u{1F9D9}',
            name: "MAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9DA}',
            name: "FAIRY".to_string(),
        },
        Emoji {
            unicode: '\u{1F9DB}',
            name: "VAMPIRE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9DC}',
            name: "MERPERSON".to_string(),
        },
        Emoji {
            unicode: '\u{1F9DD}',
            name: "ELF".to_string(),
        },
        Emoji {
            unicode: '\u{1F9DE}',
            name: "GENIEa".to_string(),
        },
        Emoji {
            unicode: '\u{1F9DF}',
            name: "ZOMBIE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E0}',
            name: "BRAIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E1}',
            name: "ORANGE HEART".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E2}',
            name: "BILLED CAP".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E3}',
            name: "SCARF".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E4}',
            name: "GLOVES".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E5}',
            name: "COAT".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E6}',
            name: "SOCKS".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E7}',
            name: "RED GIFT ENVELOPE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E8}',
            name: "FIRECRACKER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9E9}',
            name: "JIGSAW PUZZLE PIECE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9EA}',
            name: "TEST TUBE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9EB}',
            name: "PETRI DISH".to_string(),
        },
        Emoji {
            unicode: '\u{1F9EC}',
            name: "DNA DOUBLE HELIX".to_string(),
        },
        Emoji {
            unicode: '\u{1F9ED}',
            name: "COMPASS".to_string(),
        },
        Emoji {
            unicode: '\u{1F9EE}',
            name: "ABACUS".to_string(),
        },
        Emoji {
            unicode: '\u{1F9EF}',
            name: "FIRE EXTINGUISHER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F0}',
            name: "TOOLBOX".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F1}',
            name: "BRICK".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F2}',
            name: "MAGNET".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F3}',
            name: "LUGGAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F4}',
            name: "LOTION BOTTLE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F5}',
            name: "SPOOL OF THREAD".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F6}',
            name: "BALL OF YARN".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F7}',
            name: "SAFETY PIN".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F8}',
            name: "TEDDY BEAR".to_string(),
        },
        Emoji {
            unicode: '\u{1F9F9}',
            name: "BROOM".to_string(),
        },
        Emoji {
            unicode: '\u{1F9FA}',
            name: "BASKET".to_string(),
        },
        Emoji {
            unicode: '\u{1F9FB}',
            name: "ROLL OF PAPER".to_string(),
        },
        Emoji {
            unicode: '\u{1F9FC}',
            name: "BAR OF SOAP".to_string(),
        },
        Emoji {
            unicode: '\u{1F9FD}',
            name: "SPONGE".to_string(),
        },
        Emoji {
            unicode: '\u{1F9FE}',
            name: "RECEIPT".to_string(),
        },
        Emoji {
            unicode: '\u{1FA70}',
            name: "BALLET SHOES".to_string(),
        },
        Emoji {
            unicode: '\u{1FA71}',
            name: "ONE-PIECE SWIMSUIT".to_string(),
        },
        Emoji {
            unicode: '\u{1FA72}',
            name: "BRIEFS".to_string(),
        },
        Emoji {
            unicode: '\u{1FA73}',
            name: "SHORTS".to_string(),
        },
        Emoji {
            unicode: '\u{1FA78}',
            name: "DROP OF BLOOD".to_string(),
        },
        Emoji {
            unicode: '\u{1FA79}',
            name: "ADHESIVE BANDAGE".to_string(),
        },
        Emoji {
            unicode: '\u{1FA7A}',
            name: "STETHOSCOPE".to_string(),
        },
        Emoji {
            unicode: '\u{1FA80}',
            name: "YO-YO".to_string(),
        },
        Emoji {
            unicode: '\u{1FA81}',
            name: "KITE".to_string(),
        },
        Emoji {
            unicode: '\u{1FA82}',
            name: "PARACHUTE".to_string(),
        },
        Emoji {
            unicode: '\u{1FA90}',
            name: "RINGED PLANET".to_string(),
        },
        Emoji {
            unicode: '\u{1FA91}',
            name: "CHAIR".to_string(),
        },
        Emoji {
            unicode: '\u{1FA92}',
            name: "RAZOR".to_string(),
        },
        Emoji {
            unicode: '\u{1FA93}',
            name: "AXE".to_string(),
        },
        Emoji {
            unicode: '\u{1FA94}',
            name: "DIYA LAMP".to_string(),
        },
        Emoji {
            unicode: '\u{1FA95}',
            name: "BANJO".to_string(),
        },
    ]
}
