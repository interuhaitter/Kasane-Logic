pub fn convert_f(z: u8, dimension: [i64; 2]) -> Vec<(u8, i64)> {
    if (dimension[0] >= 0 && dimension[1] >= 0) || (dimension[0] < 0 && dimension[1] < 0) {
        // 上下どちらかにある場合
        return convert_f_logic(z, dimension);
    } else {
        let mut result = vec![];

        result.extend(convert_f_logic(z, [dimension[0], -1]));
        result.extend(convert_f_logic(z, [0, dimension[1]]));

        return result;
    };
}

fn convert_f_logic(z: u8, dimension: [i64; 2]) -> Vec<(u8, i64)> {
    let mut current_range = Some(dimension);
    let mut now_z = z;
    let mut result = Vec::new();

    while let Some(mut target) = current_range {
        // 終了条件：範囲が縮退した or z=0
        if target[0] >= target[1] {
            result.push((now_z, target[0]));
            break;
        }

        if now_z == 0 {
            break;
        }

        // もしも処理する部分が隣まできたら
        if target[1] - target[0] == 1 {
            // 二つをまとめられるかを判定する
            if target[0] % 2 == 0 {
                // まとめられる
                result.push((now_z - 1, target[0] / 2))
            } else {
                // まとめられない
                result.push((now_z, target[0]));
                result.push((now_z, target[1]));
            }
            break;
        }

        // 左端が奇数なら個別処理
        if target[0] % 2 != 0 {
            result.push((now_z, target[0]));
            target[0] += 1;
        }

        // 右端が偶数なら個別処理
        if target[1] % 2 == 0 {
            result.push((now_z, target[1]));
            target[1] -= 1;
        }

        // 範囲が逆転したら終了
        if target[0] > target[1] {
            break;
        }

        // 次のズームレベルに範囲を縮小
        let a = target[0] / 2;
        let b = if target[1] == -1 { -1 } else { target[1] / 2 };

        if a == b {
            result.push((now_z - 1, a));
            break;
        }

        current_range = Some([a.min(b), a.max(b)]);
        now_z -= 1;
    }
    result
}
