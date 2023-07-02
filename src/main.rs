fn multiples(limit: usize) -> impl Fn(usize) -> Vec<usize> {
    return move |number| {
        let mut mults: Vec<usize> = vec![];
        let mut j = number * number;
        while j < limit {
            mults.push(j);
            j += number;
        }
        mults
    };
}

fn primes_sqrt(limit: usize) -> Vec<usize> {
    let sqrt_len: usize = f64::sqrt(limit as f64) as usize;
    let mut primes = vec![true; limit];
    let mults = multiples(sqrt_len);
    primes[0] = false;
    primes[1] = false;
    let mut r: Vec<usize> = vec![];
    r.push(2);
    mults(2).iter().for_each(|mult| primes[*mult] = false);
    let mut i = 3;
    while i <= sqrt_len {
        if primes[i] {
            r.push(i);
            mults(i).iter().for_each(|mult| primes[*mult] = false);
        }
        i += 2;
    }
    r
}

fn primes2(limit: usize) -> Vec<usize> {
    let mut primes = vec![true; limit + 1];
    let mut r = primes_sqrt(limit);
    let mults = multiples(limit);
    r.iter()
        .for_each(|prime| mults(*prime).iter().for_each(|mult| primes[*mult] = false));
    let mut i = r.last().unwrap_or(&3).clone() + 2;
    while i <= limit {
        if primes[i] {
            r.push(i);
        }
        i += 2;
    }
    r
}

fn primes(limit: usize) -> Vec<usize> {
    let mut primes = vec![true; limit + 1];
    primes[0] = false;
    primes[1] = false;
    let mut j: usize;
    let mut r: Vec<usize> = vec![];
    r.push(2);
    j = 4;
    while j <= limit {
        primes[j] = false;
        j += 2;
    }
    let mut i = 3;
    while i * i <= limit {
        if primes[i] {
            r.push(i);
            j = i * i;
            while j <= limit {
                primes[j] = false;
                j += i;
            }
        }
        i += 2;
    }
    while i <= limit {
        if primes[i] {
            r.push(i);
        }
        i += 1;
    }
    r
}

fn main() {
    let limit = 1000;
    let foo = primes2(limit);
    let bar = primes(limit);
    println!("{:?} \n vs \n{:?} \n\n {:?}", foo, bar, foo == bar);
}
