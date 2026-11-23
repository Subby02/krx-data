use std::io::{BufReader, Read};

use anyhow::Result;

fn read_arr<const N: usize>(r: &mut impl Read) -> Result<[u8; N]> {
    let mut buf = [0u8; N];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
pub struct A0Body {
    pub seq_no: [u8; 8],             // 정보분배일련번호
    pub total_issue_cnt: [u8; 6],    // 정보구분총종목수
    pub biz_date: [u8; 8],           // 영업일자
    pub isin: [u8; 12],              // 종목코드
    pub isin_idx: [u8; 6],           // 정보분배종목인덱스
    pub short_code: [u8; 9],         // 종목단축코드
    pub name_kr: [u8; 40],           // 종목약명
    pub name_en: [u8; 40],           // 종목영문약명
    pub group_no: [u8; 5],           // 정보분배그룹번호
    pub prod_group_id: [u8; 3],      // 장운영상품그룹ID
    pub sec_group_id: [u8; 2],       // 증권그룹ID
    pub is_unit_trade: [u8; 1],      // 단위매매체결여부
    pub ex_right_code: [u8; 2],      // 락구분코드
    pub par_chg_code: [u8; 2],       // 액면가변경구분코드
    pub is_open_base: [u8; 1],       // 시가기준가격종목여부
    pub reval_code: [u8; 2],         // 재평가종목사유코드
    pub is_base_chg: [u8; 1],        // 기준가격변경종목여부
    pub rand_end_code: [u8; 1],      // 임의종료발동조건코드
    pub is_alert_notice: [u8; 1],    // 시장경보위험예고여부
    pub alert_code: [u8; 2],         // 시장경보구분코드
    pub is_good_governance: [u8; 1], // 지배구조우량여부
    pub is_admin: [u8; 1],           // 관리종목여부
    pub is_unfaithful: [u8; 1],      // 불성실공시여부
    pub is_backdoor: [u8; 1],        // 우회상장여부
    pub is_halted: [u8; 1],          // 거래정지여부
    pub industry_id: [u8; 10],       // 업종ID
    pub is_sme: [u8; 1],             // 중소기업여부
    pub section_code: [u8; 1],       // 소속부구분코드
    pub vehicle_code: [u8; 1],       // 투자기구구분코드
    pub base_price: [u8; 11],        // 기준가격
    pub prev_close_code: [u8; 1],    // 전일종가구분코드
    pub prev_close: [u8; 11],        // 전일종가
    pub prev_volume: [u8; 12],       // 전일누적거래량
    pub prev_amount: [u8; 22],       // 전일누적거래대금
    pub upper_limit: [u8; 11],       // 상한가
    pub lower_limit: [u8; 11],       // 하한가
    pub subst_price: [u8; 11],       // 대용가격
    pub par_value: [u8; 11],         // 액면가
    pub issue_price: [u8; 11],       // 발행가격
    pub listing_date: [u8; 8],       // 상장일자
    pub listed_shares: [u8; 16],     // 상장주식수
    pub is_liquidation: [u8; 1],     // 정리매매여부
    pub exist_start: [u8; 8],        // 존립개시일자
    pub exist_end: [u8; 8],          // 존립종료일자
    pub exer_start: [u8; 8],         // 행사기간개시일자
    pub exer_end: [u8; 8],           // 권리행사기간만료일자
    pub strike_price: [u8; 13],      // ELW신주인수권증권행사가격
    pub capital: [u8; 22],           // 자본금
    pub is_credit: [u8; 1],          // 신용주문가능여부
    pub limit_cond: [u8; 5],         // 지정가호가취소조건코드
    pub market_cond: [u8; 5],        // 시장가호가취소조건코드
    pub cond_limit_cond: [u8; 5],    // 조건부지정가호가취소조건코드
    pub best_limit_cond: [u8; 5],    // 최유리지정가호가취소조건코드
    pub first_limit_cond: [u8; 5],   // 최우선지정가호가취소조건코드
    pub mid_price_cond: [u8; 5],     // 중간가호가취소조건코드
    pub stop_limit_cond: [u8; 5],    // 스톱지정가호가취소조건코드
    pub cap_incr_code: [u8; 2],      // 증자구분코드
    pub share_class_code: [u8; 1],   // 종류주권구분코드
    pub is_national: [u8; 1],        // 국민주여부
    pub eval_price: [u8; 11],        // 평가가격
    pub min_quote: [u8; 11],         // 최저호가가격
    pub max_quote: [u8; 11],         // 최고호가가격
    pub lot_size: [u8; 11],          // 메인보드수량단위
    pub after_lot_size: [u8; 11],    // 장종료후시간외수량단위
    pub reits_code: [u8; 1],         // 리츠종류코드
    pub target_code: [u8; 12],       // 목적주권종목코드
    pub currency: [u8; 3],           // 통화ISO코드
    pub country: [u8; 3],            // 국가코드
    pub is_mm: [u8; 1],              // 시장조성가능여부
    pub is_post_close: [u8; 1],      // 장종료후시간외종가가능여부
    pub is_pre_close: [u8; 1],       // 장개시전시간외종가가능여부
    pub is_pre_block: [u8; 1],       // 장개시전시간외대량매매가능여부
    pub is_pre_basket: [u8; 1],      // 장개시전시간외바스켓가능여부
    pub is_exp_price_open: [u8; 1],  // 예상체결가공개여부
    pub is_short_sell: [u8; 1],      // 공매도가능여부
    pub track_multiple: [u8; 13],    // 추적수익률배수
    pub is_regs: [u8; 1],            // REGS여부
    pub is_spac: [u8; 1],            // SPAC여부
    pub tax_code: [u8; 1],           // 과세유형코드
    pub subst_rate: [u8; 13],        // 대용가격사정비율
    pub is_caution_alert: [u8; 1],   // 투자주의환기종목여부
    pub delisting_date: [u8; 8],     // 상장폐지일자
    pub overheat_code: [u8; 1],      // 단기과열종목구분코드
    pub replication_code: [u8; 1],   // ETF복제방법구분코드
    pub maturity_date: [u8; 8],      // 만기일자
    pub dist_code: [u8; 2],          // 분배금형태코드
    pub redeem_start: [u8; 8],       // 만기상환가격결정시작일자
    pub redeem_end: [u8; 8],         // 만기상환가격결정종료일자
    pub etp_code: [u8; 1],           // ETP상품구분코드
    pub idx_provider: [u8; 2],       // 지수산출기관코드
    pub idx_market_id: [u8; 6],      // 지수시장분류ID
    pub idx_seq_no: [u8; 3],         // 지수일련번호
    pub track_lev_code: [u8; 2],     // 추적지수레버리지인버스구분코드
    pub ref_lev_code: [u8; 2],       // 참고지수레버리지인버스구분코드
    pub idx_asset_id1: [u8; 6],      // 지수자산분류ID1
    pub idx_asset_id2: [u8; 6],      // 지수자산분류ID2
    pub sponsor_no: [u8; 5],         // 상장주선사회원번호
    pub is_lp_order: [u8; 1],        // LP주문가능여부
    pub is_low_liquidity: [u8; 1],   // 저유동성여부
    pub is_surge: [u8; 1],           // 이상급등여부
    pub max_qty: [u8; 23],           // 상한수량
    pub is_caution: [u8; 1],         // 투자유의종목여부
    pub is_low_shares: [u8; 1],      // 상장주식수부족종목여부
    pub is_spac_merge: [u8; 1],      // SPAC소멸합병여부
    pub segment_code: [u8; 1],       // 세그먼트구분코드
    pub is_after_market: [u8; 1],    // 애프터마켓가능여부
    pub is_pre_market: [u8; 1],      // 프리마켓가능여부
}

impl A0Body {
    pub fn new(r: &mut BufReader<&[u8]>) -> Result<Self> {
        Ok(Self {
            seq_no: read_arr(r)?,
            total_issue_cnt: read_arr(r)?,
            biz_date: read_arr(r)?,
            isin: read_arr(r)?,
            isin_idx: read_arr(r)?,
            short_code: read_arr(r)?,
            name_kr: read_arr(r)?,
            name_en: read_arr(r)?,
            group_no: read_arr(r)?,
            prod_group_id: read_arr(r)?,
            sec_group_id: read_arr(r)?,
            is_unit_trade: read_arr(r)?,
            ex_right_code: read_arr(r)?,
            par_chg_code: read_arr(r)?,
            is_open_base: read_arr(r)?,
            reval_code: read_arr(r)?,
            is_base_chg: read_arr(r)?,
            rand_end_code: read_arr(r)?,
            is_alert_notice: read_arr(r)?,
            alert_code: read_arr(r)?,
            is_good_governance: read_arr(r)?,
            is_admin: read_arr(r)?,
            is_unfaithful: read_arr(r)?,
            is_backdoor: read_arr(r)?,
            is_halted: read_arr(r)?,
            industry_id: read_arr(r)?,
            is_sme: read_arr(r)?,
            section_code: read_arr(r)?,
            vehicle_code: read_arr(r)?,
            base_price: read_arr(r)?,
            prev_close_code: read_arr(r)?,
            prev_close: read_arr(r)?,
            prev_volume: read_arr(r)?,
            prev_amount: read_arr(r)?,
            upper_limit: read_arr(r)?,
            lower_limit: read_arr(r)?,
            subst_price: read_arr(r)?,
            par_value: read_arr(r)?,
            issue_price: read_arr(r)?,
            listing_date: read_arr(r)?,
            listed_shares: read_arr(r)?,
            is_liquidation: read_arr(r)?,
            exist_start: read_arr(r)?,
            exist_end: read_arr(r)?,
            exer_start: read_arr(r)?,
            exer_end: read_arr(r)?,
            strike_price: read_arr(r)?,
            capital: read_arr(r)?,
            is_credit: read_arr(r)?,
            limit_cond: read_arr(r)?,
            market_cond: read_arr(r)?,
            cond_limit_cond: read_arr(r)?,
            best_limit_cond: read_arr(r)?,
            first_limit_cond: read_arr(r)?,
            mid_price_cond: read_arr(r)?,
            stop_limit_cond: read_arr(r)?,
            cap_incr_code: read_arr(r)?,
            share_class_code: read_arr(r)?,
            is_national: read_arr(r)?,
            eval_price: read_arr(r)?,
            min_quote: read_arr(r)?,
            max_quote: read_arr(r)?,
            lot_size: read_arr(r)?,
            after_lot_size: read_arr(r)?,
            reits_code: read_arr(r)?,
            target_code: read_arr(r)?,
            currency: read_arr(r)?,
            country: read_arr(r)?,
            is_mm: read_arr(r)?,
            is_post_close: read_arr(r)?,
            is_pre_close: read_arr(r)?,
            is_pre_block: read_arr(r)?,
            is_pre_basket: read_arr(r)?,
            is_exp_price_open: read_arr(r)?,
            is_short_sell: read_arr(r)?,
            track_multiple: read_arr(r)?,
            is_regs: read_arr(r)?,
            is_spac: read_arr(r)?,
            tax_code: read_arr(r)?,
            subst_rate: read_arr(r)?,
            is_caution_alert: read_arr(r)?,
            delisting_date: read_arr(r)?,
            overheat_code: read_arr(r)?,
            replication_code: read_arr(r)?,
            maturity_date: read_arr(r)?,
            dist_code: read_arr(r)?,
            redeem_start: read_arr(r)?,
            redeem_end: read_arr(r)?,
            etp_code: read_arr(r)?,
            idx_provider: read_arr(r)?,
            idx_market_id: read_arr(r)?,
            idx_seq_no: read_arr(r)?,
            track_lev_code: read_arr(r)?,
            ref_lev_code: read_arr(r)?,
            idx_asset_id1: read_arr(r)?,
            idx_asset_id2: read_arr(r)?,
            sponsor_no: read_arr(r)?,
            is_lp_order: read_arr(r)?,
            is_low_liquidity: read_arr(r)?,
            is_surge: read_arr(r)?,
            max_qty: read_arr(r)?,
            is_caution: read_arr(r)?,
            is_low_shares: read_arr(r)?,
            is_spac_merge: read_arr(r)?,
            segment_code: read_arr(r)?,
            is_after_market: read_arr(r)?,
            is_pre_market: read_arr(r)?,
        })
    }
}
