pub(crate) fn segment_f(z: u8, dimension: [i64; 2]) -> Vec<(u8, i64)> {
    let diff = 2_i64.pow(z.into());
    let mut target = [dimension[0] + diff, dimension[1] + diff];
    let mut now_z = z;
    let mut result = Vec::new();

    loop {
        if target[0] == target[1] {
            // 終端 → これ以上分割しない
            result.push((now_z, target[0] - 2_i64.pow(now_z.into())));
            break;
        }

        // 左端が奇数なら個別に処理
        if target[0] % 2 != 0 {
            result.push((now_z, target[0] - 2_i64.pow(now_z.into())));
            target[0] += 1;
        }

        // 右端が偶数なら個別に処理
        if target[1] % 2 == 0 {
            result.push((now_z, target[1] - 2_i64.pow(now_z.into())));
            target[1] -= 1;
        }

        // 範囲が逆転したら終了
        if target[0] > target[1] {
            break;
        }

        // 次のズームレベルへ（範囲を半分に縮小）
        if now_z == 0 {
            break; // z=0で終了
        }

        target = [target[0] / 2, target[1] / 2];
        now_z -= 1;
    }

    result
}

pub(crate) fn segment_xy(z: u8, dimension: [u64; 2]) -> Vec<(u8, u64)> {
    let mut target = dimension;
    let mut now_z = z;
    let mut result = Vec::new();

    loop {
        if target[0] == target[1] {
            // 終端 → これ以上分割しない
            result.push((now_z, target[0]));
            break;
        }

        // 左端が奇数なら個別に処理
        if target[0] % 2 != 0 {
            result.push((now_z, target[0]));
            target[0] += 1;
        }

        // 右端が偶数なら個別に処理
        if target[1] % 2 == 0 {
            result.push((now_z, target[1]));
            target[1] -= 1;
        }

        // 範囲が逆転したら終了
        if target[0] > target[1] {
            break;
        }

        // 次のズームレベルへ（範囲を半分に縮小）
        if now_z == 0 {
            break; // z=0で終了
        }

        target = [target[0] / 2, target[1] / 2];
        now_z -= 1;
    }

    result
}
