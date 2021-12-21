#[derive(Clone)]
struct Pawn {
    pos: i64,
    score: i64,
}

struct Dice {
    roll: i64,
}

fn main() {
    //Part 1
    let mut player1 = Pawn { pos: 7, score: 0 };
    let mut player2 = Pawn { pos: 6, score: 0 };
    let mut die = Dice { roll: 1 };

    let mut player1_s_turn = true;
    let mut nr_of_dice_rolls = 0;
    while player1.score < 1000 && player2.score < 1000 {
        let mut die_sum = 0;
        for _ in 0..3 {
            die_sum += die.roll;
            die.roll += 1;
            if die.roll == 101 {
                die.roll = 1;
            };
            nr_of_dice_rolls += 1;
        }

        if player1_s_turn {
            while die_sum > 0 {
                die_sum -= 1;
                player1.pos += 1;
                if player1.pos == 11 {
                    player1.pos = 1;
                }
            }
            player1.score += player1.pos;
        } else {
            while die_sum > 0 {
                die_sum -= 1;
                player2.pos += 1;
                if player2.pos == 11 {
                    player2.pos = 1;
                }
            }
            player2.score += player2.pos;
        }
        player1_s_turn = !player1_s_turn;
    }

    println!(
        "Player 1 score{}, player2 score: {}, rolls: {}",
        player1.score, player2.score, nr_of_dice_rolls
    );

    let player1 = Pawn { pos: 7, score: 0 };
    let player2 = Pawn { pos: 6, score: 0 };
    let result = play_dirac(player1, player2, true, 0, 0);
    println!("Result: {:?}", result);
}

fn play_dirac(
    mut p1: Pawn,
    mut p2: Pawn,
    p1sturn: bool,
    nr_of_roll: i64,
    mut sum_of_roll: i64,
) -> (i64, i64) {
    let mut sum: (i64, i64) = (0, 0);
    if nr_of_roll == 3 {
        if p1sturn {
            while sum_of_roll > 0 {
                sum_of_roll -= 1;
                p1.pos += 1;
                if p1.pos == 11 {
                    p1.pos = 1;
                }
            }
            p1.score += p1.pos;
        } else {
            while sum_of_roll > 0 {
                sum_of_roll -= 1;
                p2.pos += 1;
                if p2.pos == 11 {
                    p2.pos = 1;
                }
            }
            p2.score += p2.pos;
        }

        if p1.score < 21 && p2.score < 21 {
            let playdirac = play_dirac(p1, p2, !p1sturn, 0, 0);
            sum.0 += playdirac.0;
            sum.1 += playdirac.1;
        } else {
            if p1.score >= 21 {
                return (1, 0);
            } else {
                return (0, 1);
            }
        }
    } else {
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 3);
        sum.0 += playdirac.0;
        sum.1 += playdirac.1;
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 4);
        sum.0 += playdirac.0 * 3;
        sum.1 += playdirac.1 * 3;
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 5);
        sum.0 += playdirac.0 * 6;
        sum.1 += playdirac.1 * 6;
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 6);
        sum.0 += playdirac.0 * 7;
        sum.1 += playdirac.1 * 7;
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 7);
        sum.0 += playdirac.0 * 6;
        sum.1 += playdirac.1 * 6;
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 8);
        sum.0 += playdirac.0 * 3;
        sum.1 += playdirac.1 * 3;
        let playdirac = play_dirac(p1.clone(), p2.clone(), p1sturn, 3, 9);
        sum.0 += playdirac.0;
        sum.1 += playdirac.1;
    }
    return sum;
}
