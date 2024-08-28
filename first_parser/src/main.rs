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

enum MyResult {
  Changed(Term),
  Unchanged(Term)
}

impl MyResult {
  fn flat_map(self) -> Term {
    match self {
        MyResult::Changed(value) => value,
        MyResult::Unchanged(value) => value,
    }
  }
}

fn eval1(t: Term) -> MyResult {
  match t {
    Term::TmIf(_, t1, t2, t3) => {
      match *t1 {
        Term::TmTrue(_) => MyResult::Changed(*t2),
        Term::TmFalse(_) => MyResult::Changed(*t3),
        _ => MyResult::Changed(Term::TmIf(Info { line: 0 }, Box::new(eval1(*t1).flat_map()), t2, t3)),
      }
    }
    Term::TmSucc(_, t1) => MyResult::Changed(Term::TmSucc(Info { line: 0 }, Box::new(eval1(*t1).flat_map()))),
    Term::TmPred(_, t1) => {
      match *t1 {
        Term::TmZero(_) => MyResult::Changed(Term::TmZero(Info { line: 0 })),
        Term::TmSucc(_, nv1) if isnumericval(&*nv1) => MyResult::Changed(*nv1),
        _ => MyResult::Changed(Term::TmPred(Info { line: 0 }, Box::new(eval1(*t1).flat_map()))),
      }
    }
    Term::TmIsZero(_, t1) => {
      match *t1 {
        Term::TmZero(_) => MyResult::Changed(Term::TmTrue(Info { line: 0 })),
        Term::TmSucc(_, nv1) if isnumericval(&*nv1) => MyResult::Changed(Term::TmFalse(Info { line: 0 })),
        _ => MyResult::Changed(Term::TmIsZero(Info { line: 0 }, Box::new(eval1(*t1).flat_map()))),
      }
    }
    _ => MyResult::Unchanged(t),
  }
}

fn eval(t: Term) -> Term {
  let t1 = eval1(t);
  match t1 {
    MyResult::Changed(term) => eval(term),
    MyResult::Unchanged(term) => term,
  }
}



fn main() {
    println!("Hello, world!");
}
