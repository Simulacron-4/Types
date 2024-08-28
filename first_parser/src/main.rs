#[derive(PartialEq)]
struct Info {
  line: u32,
}

#[derive(PartialEq)]
enum Term {
  TmTrue(Info),
  TmFalse(Info),
  TmIf(Info, Box<Term>, Box<Term>, Box<Term>),
  TmZero(Info),
  TmSucc(Info, Box<Term>),
  TmPred(Info, Box<Term>),
  TmIsZero(Info, Box<Term>),
}

fn isnumericval(t: &Term) -> bool {
  match t {
    Term::TmZero(_) => true,
    Term::TmSucc(_, t1) => isnumericval(t1),
    _ => false,
  }
}

fn isval(t: &Term) -> bool {
  match t {
    Term::TmTrue(_) => true,
    Term::TmFalse(_) => true,
    t if isnumericval(t) => true,
    _ => false,
  }
}

fn eval1(t: Term) -> Term {
  match t {
    Term::TmIf(_, t1, t2, t3) => {
      match *t1 {
        Term::TmTrue(_) => *t2,
        Term::TmFalse(_) => *t3,
        _ => Term::TmIf(Info { line: 0 }, Box::new(eval1(*t1)), t2, t3),
      }
    }
    Term::TmSucc(_, t1) => Term::TmSucc(Info { line: 0 }, Box::new(eval1(*t1))),
    Term::TmPred(_, t1) => {
      match *t1 {
        Term::TmZero(_) => Term::TmZero(Info { line: 0 }),
        Term::TmSucc(_, nv1) if isnumericval(&*nv1) => *nv1,
        _ => Term::TmPred(Info { line: 0 }, Box::new(eval1(*t1))),
      }
    }
    Term::TmIsZero(_, t1) => {
      match *t1 {
        Term::TmZero(_) => Term::TmTrue(Info { line: 0 }),
        Term::TmSucc(_, nv1) if isnumericval(&*nv1) => Term::TmFalse(Info { line: 0 }),
        _ => Term::TmIsZero(Info { line: 0 }, Box::new(eval1(*t1))),
      }
    }
    _ => t,
  }
}

/*
fn eval(t: Term) -> Term {
  let t1 = eval1(t);
  if t1 == t {
    t
  } else {
    eval(t1)
  }
}
*/



fn main() {
    println!("Hello, world!");
}
