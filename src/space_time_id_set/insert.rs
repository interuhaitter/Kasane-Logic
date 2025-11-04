use crate::{
    space_time_id::SpaceTimeId,
    space_time_id_set::{
        SpaceTimeIdSet,
        single::{
            convert_bitvec_f::convert_bitmask_f,
            convert_bitvec_xy::convert_bitmask_xy,
            convert_single_f::{self, convert_f},
            convert_single_xy::convert_xy,
        },
    },
    r#type::bit_vec::BitVec,
};

impl SpaceTimeIdSet {
    pub fn insert(&mut self, id: SpaceTimeId) {
        //IDを各次元ごとに最適な単体範囲に分解する
        let f_splited = convert_f(id.z, id.f);
        let x_splited = convert_xy(id.z, id.x);
        let y_splited = convert_xy(id.z, id.y);

        //SplitしたIDをデバッグ用に表示する
        #[cfg(debug_assertions)]
        {
            for (z, f) in &f_splited {
                println!("{}/{}/-/-,", z, f);
            }
            for (z, x) in &x_splited {
                println!("{}/-/{}/-,", z, x);
            }
            for (z, y) in &y_splited {
                println!("{}/-/-/{},", z, y);
            }
        }

        //各次元の範囲をBitVecに変換する
        let f_encoded: Vec<BitVec> = f_splited
            .iter()
            .map(|(z, f)| convert_bitmask_f(*z, *f))
            .collect();
        let x_encoded: Vec<BitVec> = x_splited
            .iter()
            .map(|(z, x)| convert_bitmask_xy(*z, *x))
            .collect();
        let y_encoded: Vec<BitVec> = y_splited
            .iter()
            .map(|(z, y)| convert_bitmask_xy(*z, *y))
            .collect();

        //SplitしたIDをデバッグ用に表示する
        #[cfg(debug_assertions)]
        {
            for f in &f_encoded {
                println!("{}", f);
            }
            for x in &x_encoded {
                println!("{}", x);
            }
            for y in &y_encoded {
                println!("{}", y);
            }
        }

        #[cfg(debug_assertions)]
        {
            use crate::space_time_id_set::single::invert_bitvec_f::invert_bitmask_f;
            use crate::space_time_id_set::single::invert_bitvec_xy::invert_bitmask_xy;

            for f in &f_encoded {
                let decode = invert_bitmask_f(f);
                println!("{}/{}/-/-,", decode.0, decode.1);
            }
            for x in &x_encoded {
                let decode = invert_bitmask_xy(x);
                println!("{}/-/{}/-,", decode.0, decode.1);
            }
            for y in &y_encoded {
                let decode = invert_bitmask_xy(y);
                println!("{}/-/-/{},", decode.0, decode.1);
            }
        }

        //分離範囲ごとに下位IDの個数を調べる

        //下位IDの個数が少ない順にSortする

        //挿入していく

        //上位IDを調べる

        //上位IDがある場合は逆引きして他の次元と重なりがないかを検証する

        //この段階で代表次元について上位IDと下位IDが出そろう
        //順番に逆引きしていく
        //上位IDの場合は挿入しない
        //下位IDの場合は下位IDを削除
        //部分の場合は総合して下位を切る
        //隣に連続なIDがあればくっつける

        //これを繰り返す
    }
}
