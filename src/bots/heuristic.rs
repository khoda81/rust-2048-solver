use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::board::{Board, Cell};

#[derive(Copy, Clone, Debug, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmptyCount(pub u8);
#[derive(Copy, Clone, Debug, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxCell(pub Cell);
pub type PreprocessedBoard = (EmptyCount, MaxCell);
pub type Eval = f64;

pub fn preprocess_board<const ROWS: usize, const COLS: usize>(
    board: &Board<ROWS, COLS>,
) -> (EmptyCount, MaxCell) {
    (
        EmptyCount(board.count_empty() as u8),
        MaxCell(board.into_iter().flatten().max().unwrap() as Cell),
    )
}

pub fn generate_lookup() -> HashMap<PreprocessedBoard, Eval> {
    let mut map = HashMap::new();

    map.insert((EmptyCount(11), MaxCell(1)), 3469.7003173828125);
    map.insert((EmptyCount(12), MaxCell(1)), 3610.103654191301);
    map.insert((EmptyCount(13), MaxCell(1)), 3613.6753993890225);
    map.insert((EmptyCount(14), MaxCell(1)), 3561.7130533854165);
    map.insert((EmptyCount(15), MaxCell(1)), 3698.61376953125);
    map.insert((EmptyCount(4), MaxCell(2)), 2414.547848791079);
    map.insert((EmptyCount(5), MaxCell(2)), 2598.564676885849);
    map.insert((EmptyCount(6), MaxCell(2)), 3101.829523945135);
    map.insert((EmptyCount(7), MaxCell(2)), 3081.43973119993);
    map.insert((EmptyCount(8), MaxCell(2)), 3062.7831637490453);
    map.insert((EmptyCount(9), MaxCell(2)), 3023.053374395352);
    map.insert((EmptyCount(10), MaxCell(2)), 3116.2887703019346);
    map.insert((EmptyCount(11), MaxCell(2)), 3122.3085594126323);
    map.insert((EmptyCount(12), MaxCell(2)), 3133.699252690822);
    map.insert((EmptyCount(13), MaxCell(2)), 3274.0390164608775);
    map.insert((EmptyCount(14), MaxCell(2)), 3113.8981962144);
    map.insert((EmptyCount(15), MaxCell(2)), 3214.7108366087255);
    map.insert((EmptyCount(0), MaxCell(3)), 3063.025374081727);
    map.insert((EmptyCount(1), MaxCell(3)), 3032.5520429953176);
    map.insert((EmptyCount(2), MaxCell(3)), 3041.4120273509266);
    map.insert((EmptyCount(3), MaxCell(3)), 3038.7713909513363);
    map.insert((EmptyCount(4), MaxCell(3)), 3039.7455029822418);
    map.insert((EmptyCount(5), MaxCell(3)), 3045.0627730269202);
    map.insert((EmptyCount(6), MaxCell(3)), 3045.571277949135);
    map.insert((EmptyCount(7), MaxCell(3)), 3044.6932673072156);
    map.insert((EmptyCount(8), MaxCell(3)), 3056.159761281611);
    map.insert((EmptyCount(9), MaxCell(3)), 3059.813142448239);
    map.insert((EmptyCount(10), MaxCell(3)), 3066.1077720490266);
    map.insert((EmptyCount(11), MaxCell(3)), 3065.4712119346);
    map.insert((EmptyCount(12), MaxCell(3)), 3072.2003740117307);
    map.insert((EmptyCount(13), MaxCell(3)), 3105.430231906674);
    map.insert((EmptyCount(14), MaxCell(3)), 3362.099300218255);
    map.insert((EmptyCount(0), MaxCell(4)), 2808.89036039599);
    map.insert((EmptyCount(1), MaxCell(4)), 2780.6488646875123);
    map.insert((EmptyCount(2), MaxCell(4)), 2861.082977086879);
    map.insert((EmptyCount(3), MaxCell(4)), 2959.4445147232846);
    map.insert((EmptyCount(4), MaxCell(4)), 3037.7615312692706);
    map.insert((EmptyCount(5), MaxCell(4)), 3009.341571175092);
    map.insert((EmptyCount(6), MaxCell(4)), 2983.7938799636945);
    map.insert((EmptyCount(7), MaxCell(4)), 3013.5292130925104);
    map.insert((EmptyCount(8), MaxCell(4)), 3031.775214638621);
    map.insert((EmptyCount(9), MaxCell(4)), 3033.82284359294);
    map.insert((EmptyCount(10), MaxCell(4)), 3028.051628084571);
    map.insert((EmptyCount(11), MaxCell(4)), 3026.5120598898093);
    map.insert((EmptyCount(12), MaxCell(4)), 3015.2722436032755);
    map.insert((EmptyCount(13), MaxCell(4)), 3013.0379426748987);
    map.insert((EmptyCount(0), MaxCell(5)), 2088.9237111115544);
    map.insert((EmptyCount(1), MaxCell(5)), 2457.441088376341);
    map.insert((EmptyCount(2), MaxCell(5)), 2782.833617414055);
    map.insert((EmptyCount(3), MaxCell(5)), 2750.354445019195);
    map.insert((EmptyCount(4), MaxCell(5)), 2737.090758917239);
    map.insert((EmptyCount(5), MaxCell(5)), 2803.935951634325);
    map.insert((EmptyCount(6), MaxCell(5)), 2817.57301104161);
    map.insert((EmptyCount(7), MaxCell(5)), 2817.033097774833);
    map.insert((EmptyCount(8), MaxCell(5)), 2814.847370974524);
    map.insert((EmptyCount(9), MaxCell(5)), 2791.624547682743);
    map.insert((EmptyCount(10), MaxCell(5)), 2655.759113006457);
    map.insert((EmptyCount(11), MaxCell(5)), 2644.951512401021);
    map.insert((EmptyCount(12), MaxCell(5)), 1976.9374422316691);
    map.insert((EmptyCount(13), MaxCell(5)), 1839.8732081821986);
    map.insert((EmptyCount(0), MaxCell(6)), 2212.4509605294147);
    map.insert((EmptyCount(1), MaxCell(6)), 1992.77901297143);
    map.insert((EmptyCount(2), MaxCell(6)), 2397.036934888557);
    map.insert((EmptyCount(3), MaxCell(6)), 2410.491735023492);
    map.insert((EmptyCount(4), MaxCell(6)), 2412.027242272843);
    map.insert((EmptyCount(5), MaxCell(6)), 2405.877667776546);
    map.insert((EmptyCount(6), MaxCell(6)), 2412.6403715504543);
    map.insert((EmptyCount(7), MaxCell(6)), 2415.899490110177);
    map.insert((EmptyCount(8), MaxCell(6)), 2447.1968036815724);
    map.insert((EmptyCount(9), MaxCell(6)), 2084.708801627474);
    map.insert((EmptyCount(10), MaxCell(6)), 1971.8402052407216);
    map.insert((EmptyCount(11), MaxCell(6)), 2099.0709807661747);
    map.insert((EmptyCount(12), MaxCell(6)), 2298.794189453125);
    map.insert((EmptyCount(0), MaxCell(7)), 1095.8994803099276);
    map.insert((EmptyCount(1), MaxCell(7)), 973.356392931131);
    map.insert((EmptyCount(2), MaxCell(7)), 1299.8832342696999);
    map.insert((EmptyCount(3), MaxCell(7)), 1428.204445175114);
    map.insert((EmptyCount(4), MaxCell(7)), 1642.3371866561204);
    map.insert((EmptyCount(5), MaxCell(7)), 2123.6968880618188);
    map.insert((EmptyCount(6), MaxCell(7)), 2367.131627509567);
    map.insert((EmptyCount(7), MaxCell(7)), 2450.7763735559);
    map.insert((EmptyCount(8), MaxCell(7)), 2463.517371541917);
    map.insert((EmptyCount(9), MaxCell(7)), 2464.387936881673);
    map.insert((EmptyCount(10), MaxCell(7)), 2376.3505267154173);
    map.insert((EmptyCount(0), MaxCell(8)), 1179.47517034642);
    map.insert((EmptyCount(1), MaxCell(8)), 802.6325983017468);
    map.insert((EmptyCount(2), MaxCell(8)), 1182.1490986607882);
    map.insert((EmptyCount(3), MaxCell(8)), 1452.2123657903426);
    map.insert((EmptyCount(4), MaxCell(8)), 1555.105725704206);
    map.insert((EmptyCount(5), MaxCell(8)), 1592.1275054317705);
    map.insert((EmptyCount(6), MaxCell(8)), 1552.8763361367485);
    map.insert((EmptyCount(7), MaxCell(8)), 1507.4727632869417);
    map.insert((EmptyCount(8), MaxCell(8)), 1496.6638873207935);
    map.insert((EmptyCount(9), MaxCell(8)), 1488.577307189464);
    map.insert((EmptyCount(10), MaxCell(8)), 1349.6475321432958);
    map.insert((EmptyCount(11), MaxCell(8)), 1398.126708984375);
    map.insert((EmptyCount(0), MaxCell(9)), 900.4807920794918);
    map.insert((EmptyCount(1), MaxCell(9)), 595.108173190689);
    map.insert((EmptyCount(2), MaxCell(9)), 872.5780979302324);
    map.insert((EmptyCount(3), MaxCell(9)), 1073.9791783023034);
    map.insert((EmptyCount(4), MaxCell(9)), 1133.2431311846271);
    map.insert((EmptyCount(5), MaxCell(9)), 1299.6142730227825);
    map.insert((EmptyCount(6), MaxCell(9)), 1520.515630843815);
    map.insert((EmptyCount(7), MaxCell(9)), 1561.244445195101);
    map.insert((EmptyCount(8), MaxCell(9)), 1661.7392976050105);
    map.insert((EmptyCount(9), MaxCell(9)), 1725.185813168253);
    map.insert((EmptyCount(10), MaxCell(9)), 1929.1771846364747);
    map.insert((EmptyCount(0), MaxCell(10)), 103.02655080187209);
    map.insert((EmptyCount(1), MaxCell(10)), 340.6003528616673);
    map.insert((EmptyCount(2), MaxCell(10)), 456.41458699159665);
    map.insert((EmptyCount(3), MaxCell(10)), 525.5213234443223);
    map.insert((EmptyCount(4), MaxCell(10)), 554.8176929239249);
    map.insert((EmptyCount(5), MaxCell(10)), 577.411026345761);
    map.insert((EmptyCount(6), MaxCell(10)), 613.1878562738827);
    map.insert((EmptyCount(7), MaxCell(10)), 704.4372041907573);
    map.insert((EmptyCount(8), MaxCell(10)), 722.3184204914086);
    map.insert((EmptyCount(9), MaxCell(10)), 750.450406290959);
    map.insert((EmptyCount(10), MaxCell(10)), 870.6341374515548);
    map.insert((EmptyCount(0), MaxCell(11)), 10.99386481457965);
    map.insert((EmptyCount(1), MaxCell(11)), 28.077339033875496);
    map.insert((EmptyCount(2), MaxCell(11)), 54.76746439628339);
    map.insert((EmptyCount(3), MaxCell(11)), 89.95292856867745);
    map.insert((EmptyCount(4), MaxCell(11)), 106.99087504825886);
    map.insert((EmptyCount(5), MaxCell(11)), 130.42108391289668);
    map.insert((EmptyCount(6), MaxCell(11)), 152.54940148275568);
    map.insert((EmptyCount(7), MaxCell(11)), 450.8029813351453);
    map.insert((EmptyCount(8), MaxCell(11)), 480.8050685057163);
    map.insert((EmptyCount(9), MaxCell(11)), 501.00259368654275);
    map.insert((EmptyCount(10), MaxCell(11)), 550.1526912476444);
    map.insert((EmptyCount(0), MaxCell(12)), 80.17203224999271);
    map.insert((EmptyCount(1), MaxCell(12)), 92.79287141161413);
    map.insert((EmptyCount(2), MaxCell(12)), 82.86086660698443);
    map.insert((EmptyCount(3), MaxCell(12)), 83.57377704955591);
    map.insert((EmptyCount(4), MaxCell(12)), 75.82602119495184);
    map.insert((EmptyCount(5), MaxCell(12)), 51.46136546364257);
    map.insert((EmptyCount(6), MaxCell(12)), 47.455543316518195);
    map.insert((EmptyCount(7), MaxCell(12)), 43.69681144563725);
    map.insert((EmptyCount(8), MaxCell(12)), 26.85899097818347);
    map.insert((EmptyCount(9), MaxCell(12)), 25.124716332382043);

    map
}

lazy_static! {
    static ref PRE_LOOKUP: HashMap<PreprocessedBoard, Eval> = generate_lookup();
}

pub fn get_lookup() -> &'static HashMap<PreprocessedBoard, Eval> {
    &PRE_LOOKUP
}

pub fn heuristic(preprocessed_board: PreprocessedBoard) -> Eval {
    let (empty_count, _max_cell) = preprocessed_board;

    empty_count_max_cell_lookup(preprocessed_board)
        .or_else(|| empty_count_lookup_table(empty_count))
        .unwrap_or_else(|| exponential_empty_count_heuristic(empty_count))
}

fn empty_count_max_cell_lookup(preprocessed_board: (EmptyCount, MaxCell)) -> Option<Eval> {
    get_lookup().get(&preprocessed_board).copied()
}

fn empty_count_lookup_table(EmptyCount(empty_count): EmptyCount) -> Option<Eval> {
    [
        15.82, 35.14, 752.49, 633.58, 1909.69, 3259.14, 3320.45, 3356.29, 3388.47, 3388.15,
        3446.54, 3541.35, 4071.11, 4961.21, 7341.16, 9085.73,
    ]
    .get(empty_count as usize)
    .copied()
}

fn exponential_empty_count_heuristic(EmptyCount(empty_count): EmptyCount) -> Eval {
    2_usize.pow((empty_count + 1) as u32) as Eval
}
