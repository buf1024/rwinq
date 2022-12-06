use crate::comm::async_client;
use crate::{Error, Result};
use chrono::{Duration, NaiveDate};
use js_sandbox::Script;
use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::ops::Add;
use std::sync::RwLock;

static CACHE_TRADE_DATE: Lazy<RwLock<BTreeSet<i32>>> = Lazy::new(|| {
    let set: BTreeSet<i32> = BTreeSet::new();
    RwLock::new(set)
});

pub async fn fetch_trade_date() -> Result<BTreeSet<i32>> {
    let client = async_client();

    let resp = client
        .get("https://finance.sina.com.cn/realstock/company/klc_td_sh.txt")
        .send()
        .await?
        .text()
        .await?;
    let js_code = format!("{}{}", resp, JS_CODE);
    let mut js_script =
        Script::from_string(&js_code).map_err(|_| Error::Custom("Load js code fatal!"))?;
    let data: BTreeSet<i32> = js_script
        .call("get_trade_date", &())
        .map_err(|_| Error::Custom("Call js function fatal!"))?;

    {
        let mut cache = CACHE_TRADE_DATE.write().unwrap();
        *cache = data.clone();
    }

    Ok(data)
}

pub async fn fetch_next_trade_date(date: &NaiveDate) -> Result<i32> {
    let mut mx_date = {
        let cache = CACHE_TRADE_DATE.read().unwrap();
        let elm = cache.iter().next_back();
        elm.map_or(0, |e| *e)
    };

    if mx_date == 0 {
        let cache = fetch_trade_date().await?;
        let elm = cache.iter().next_back();
        mx_date = elm.map_or(0, |e| *e);
    }

    let mut test_date = date.clone();
    loop {
        test_date = test_date.add(Duration::days(1));
        let d: i32 = test_date.format("%Y%m%d").to_string().parse().unwrap();
        if d > mx_date {
            break;
        }
        {
            let cache = CACHE_TRADE_DATE.read().unwrap();
            if cache.contains(&d) {
                return Ok(*(cache.get(&d).unwrap()));
            }
        }
    }
    Err(Error::Custom("date is to far ..."))
}
pub async fn fetch_prev_trade_date(date: &NaiveDate) -> Result<i32> {
    let mut min_date = {
        let cache = CACHE_TRADE_DATE.read().unwrap();
        let elm = cache.iter().next();
        elm.map_or(0, |e| *e)
    };

    if min_date == 0 {
        let cache = fetch_trade_date().await?;
        let elm = cache.iter().next();
        min_date = elm.map_or(0, |e| *e);
    }

    let mut test_date = date.clone();
    loop {
        test_date = test_date.add(Duration::days(-1));
        let d: i32 = test_date.format("%Y%m%d").to_string().parse().unwrap();
        if d < min_date {
            break;
        }
        {
            let cache = CACHE_TRADE_DATE.read().unwrap();
            if cache.contains(&d) {
                return Ok(*(cache.get(&d).unwrap()));
            }
        }
    }
    Err(Error::Custom("date is to old ..."))
}

const JS_CODE: &'static str = r###"
function d(t) {
    var e, i, n, r, a, o, s, l = (arguments,
            864e5), u = 7657, c = [], h = [], d = ~(3 << 30), f = 1 << 30,
        p = [0, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23, 24, 27, 29, 30], m = Math, g = function () {
            var l, u;
            for (l = 0; 64 > l; l++)
                h[l] = m.pow(2, l),
                26 > l && (c[l] = v(l + 65),
                    c[l + 26] = v(l + 97),
                10 > l && (c[l + 52] = v(l + 48)));
            for (c.push("+", "/"),
                     c = c.join(""),
                     i = t.split(""),
                     n = i.length,
                     l = 0; n > l; l++)
                i[l] = c.indexOf(i[l]);
            return r = {},
                e = o = 0,
                a = {},
                u = w([12, 6]),
                s = 63 ^ u[1],
            {
                _1479: T,
                _136: _,
                _200: S,
                _139: k,
                _197: _mi_run
            }["_" + u[0]] || function () {
                return []
            }
        }, v = String.fromCharCode, b = function (t) {
            return t === {}._
        }, N = function () {
            var t, e;
            for (t = y(),
                     e = 1; ;) {
                if (!y())
                    return e * (2 * t - 1);
                e++
            }
        }, y = function () {
            var t;
            return e >= n ? 0 : (t = i[e] & 1 << o,
                o++,
            o >= 6 && (o -= 6,
                e++),
                !!t)
        }, w = function (t, r, a) {
            var s, l, u, c, d;
            for (l = [],
                     u = 0,
                 r || (r = []),
                 a || (a = []),
                     s = 0; s < t.length; s++)
                if (c = t[s],
                    u = 0,
                    c) {
                    if (e >= n)
                        return l;
                    if (t[s] <= 0)
                        u = 0;
                    else if (t[s] <= 30) {
                        for (; d = 6 - o,
                                   d = c > d ? d : c,
                                   u |= (i[e] >> o & (1 << d) - 1) << t[s] - c,
                                   o += d,
                               o >= 6 && (o -= 6,
                                   e++),
                                   c -= d,
                                   !(0 >= c);)
                            ;
                        r[s] && u >= h[t[s] - 1] && (u -= h[t[s]])
                    } else
                        u = w([30, t[s] - 30], [0, r[s]]),
                        a[s] || (u = u[0] + u[1] * h[30]);
                    l[s] = u
                } else
                    l[s] = 0;
            return l
        }, x = function (t) {
            var e, i, n;
            for (t > 1 && (e = 0),
                     e = 0; t > e; e++)
                r.d++,
                    n = r.d % 7,
                (3 == n || 4 == n) && (r.d += 5 - n);
            return i = new Date,
                i.setTime((u + r.d) * l),
                i
        }, S = function () {
            var t, i, a, o, l;
            if (s >= 1)
                return [];
            for (r.d = w([18], [1])[0] - 1,
                     a = w([3, 3, 30, 6]),
                     r.p = a[0],
                     r.ld = a[1],
                     r.cd = a[2],
                     r.c = a[3],
                     r.m = m.pow(10, r.p),
                     r.pc = r.cd / r.m,
                     i = [],
                     t = 0; o = {
                d: 1
            },
                 y() && (a = w([3])[0],
                     0 == a ? o.d = w([6])[0] : 1 == a ? (r.d = w([18])[0],
                         o.d = 0) : o.d = a),
                     l = {
                         day: x(o.d)
                     },
                 y() && (r.ld += N()),
                     a = w([3 * r.ld], [1]),
                     r.cd += a[0],
                     l.close = r.cd / r.m,
                     i.push(l),
                 !(e >= n) && (e != n - 1 || 63 & (r.c ^ t + 1)); t++)
                ;
            return i[0].prevclose = r.pc,
                i
        }, _ = function () {
            var t, i, a, o, l, u, c, h, d, f, p;
            if (s > 2)
                return [];
            for (c = [],
                     d = {
                         v: "volume",
                         p: "price",
                         a: "avg_price"
                     },
                     r.d = w([18], [1])[0] - 1,
                     h = {
                         day: x(1)
                     },
                     a = w(1 > s ? [3, 3, 4, 1, 1, 1, 5] : [4, 4, 4, 1, 1, 1, 3]),
                     t = 0; 7 > t; t++)
                r[["la", "lp", "lv", "tv", "rv", "zv", "pp"][t]] = a[t];
            for (r.m = m.pow(10, r.pp),
                     s >= 1 ? (a = w([3, 3]),
                         r.c = a[0],
                         a = a[1]) : (a = 5,
                         r.c = 2),
                     r.pc = w([6 * a])[0],
                     h.pc = r.pc / r.m,
                     r.cp = r.pc,
                     r.da = 0,
                     r.sa = r.sv = 0,
                     t = 0; !(e >= n) && (e != n - 1 || 7 & (r.c ^ t)); t++) {
                for (l = {},
                         o = {},
                         f = r.tv ? y() : 1,
                         i = 0; 3 > i; i++)
                    if (p = ["v", "p", "a"][i],
                    (f ? y() : 0) && (a = N(),
                        r["l" + p] += a),
                        u = "v" == p && r.rv ? y() : 1,
                        a = w([3 * r["l" + p] + ("v" == p ? 7 * u : 0)], [!!i])[0] * (u ? 1 : 100),
                        o[p] = a,
                    "v" == p) {
                        if (!(l[d[p]] = a) && (s > 1 || 241 > t) && (r.zv ? !y() : 1)) {
                            o.p = 0;
                            break
                        }
                    } else
                        "a" == p && (r.da = (1 > s ? 0 : r.da) + o.a);
                r.sv += o.v,
                    l[d.p] = (r.cp += o.p) / r.m,
                    r.sa += o.v * r.cp,
                    l[d.a] = b(o.a) ? t ? c[t - 1][d.a] : l[d.p] : r.sv ? ((m.floor((r.sa * (2e3 / r.m) + r.sv) / r.sv) >> 1) + r.da) / 1e3 : l[d.p] + r.da / 1e3,
                    c.push(l)
            }
            return c[0].date = h.day,
                c[0].prevclose = h.pc,
                c
        }, T = function () {
            var t, e, i, n, a, o, l;
            if (s >= 1)
                return [];
            for (r.lv = 0,
                     r.ld = 0,
                     r.cd = 0,
                     r.cv = [0, 0],
                     r.p = w([6])[0],
                     r.d = w([18], [1])[0] - 1,
                     r.m = m.pow(10, r.p),
                     a = w([3, 3]),
                     r.md = a[0],
                     r.mv = a[1],
                     t = []; a = w([6]),
                     a.length;) {
                if (i = {
                    c: a[0]
                },
                    n = {},
                    i.d = 1,
                32 & i.c)
                    for (; ;) {
                        if (a = w([6])[0],
                        63 == (16 | a)) {
                            l = 16 & a ? "x" : "u",
                                a = w([3, 3]),
                                i[l + "_d"] = a[0] + r.md,
                                i[l + "_v"] = a[1] + r.mv;
                            break
                        }
                        if (32 & a) {
                            o = 8 & a ? "d" : "v",
                                l = 16 & a ? "x" : "u",
                                i[l + "_" + o] = (7 & a) + r["m" + o];
                            break
                        }
                        if (o = 15 & a,
                            0 == o ? i.d = w([6])[0] : 1 == o ? (r.d = o = w([18])[0],
                                i.d = 0) : i.d = o,
                            !(16 & a))
                            break
                    }
                n.date = x(i.d);
                for (o in {
                    v: 0,
                    d: 0
                })
                    b(i["x_" + o]) || (r["l" + o] = i["x_" + o]),
                    b(i["u_" + o]) && (i["u_" + o] = r["l" + o]);
                for (i.l_l = [i.u_d, i.u_d, i.u_d, i.u_d, i.u_v],
                         l = p[15 & i.c],
                     1 & i.u_v && (l = 31 - l),
                     16 & i.c && (i.l_l[4] += 2),
                         e = 0; 5 > e; e++)
                    l & 1 << 4 - e && i.l_l[e]++,
                        i.l_l[e] *= 3;
                i.d_v = w(i.l_l, [1, 0, 0, 1, 1], [0, 0, 0, 0, 1]),
                    o = r.cd + i.d_v[0],
                    n.open = o / r.m,
                    n.high = (o + i.d_v[1]) / r.m,
                    n.low = (o - i.d_v[2]) / r.m,
                    n.close = (o + i.d_v[3]) / r.m,
                    a = i.d_v[4],
                "number" == typeof a && (a = [a, a >= 0 ? 0 : -1]),
                    r.cd = o + i.d_v[3],
                    l = r.cv[0] + a[0],
                    r.cv = [l & d, r.cv[1] + a[1] + !!((r.cv[0] & d) + (a[0] & d) & f)],
                    n.volume = (r.cv[0] & f - 1) + r.cv[1] * f,
                    t.push(n)
            }
            return t
        }, k = function () {
            var t, e, i, n;
            if (s > 1)
                return [];
            for (r.l = 0,
                     n = -1,
                     r.d = w([18])[0] - 1,
                     i = w([18])[0]; r.d < i;)
                e = x(1),
                    0 >= n ? (y() && (r.l += N()),
                        n = w([3 * r.l], [0])[0] + 1,
                    t || (t = [e],
                        n--)) : t.push(e),
                    n--;
            return t
        };
    return _mi_run = function () {
        var t, i, a, o;
        if (s >= 1)
            return [];
        for (r.f = w([6])[0],
                 r.c = w([6])[0],
                 a = [],
                 r.dv = [],
                 r.dl = [],
                 t = 0; t < r.f; t++)
            r.dv[t] = 0,
                r.dl[t] = 0;
        for (t = 0; !(e >= n) && (e != n - 1 || 7 & (r.c ^ t)); t++) {
            for (o = [],
                     i = 0; i < r.f; i++)
                y() && (r.dl[i] += N()),
                    r.dv[i] += w([3 * r.dl[i]], [1])[0],
                    o[i] = r.dv[i];
            a.push(o)
        }
        return a
    }
        ,
        g()()
}
function normal_day(day) {
    var y = day.getFullYear();
    var m = day.getMonth() + 1;
    var d = day.getDate();

    return y*10000 + m*100 +d;
}
function get_trade_date() {
    var list = d(datelist);
    return list.map(normal_day);
}
"###;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::comm::hiq_trade_date::{
        fetch_next_trade_date, fetch_prev_trade_date, fetch_trade_date,
    };

    #[test]
    fn test_fetch_trade_date() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let data = fetch_trade_date().await;

                assert!(data.is_ok());

                let data = data.unwrap();

                let day = 20221114;
                assert!(data.contains(&day));

                let date = NaiveDate::parse_from_str("20221114", "%Y%m%d").unwrap();

                let next = fetch_next_trade_date(&date).await.unwrap();
                let prev = fetch_prev_trade_date(&date).await.unwrap();
                println!("next: {}, prev: {}", next, prev);
            })
    }
}
