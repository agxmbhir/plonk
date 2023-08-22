// eq - x^3 + x + 5 == 35
// Solution: x = 3; 

use ark_ff::Field;
use ark_test_curves::bls12_381::Fq2 as F;
use ark_poly::univariate::DensePolynomial as Polynomial;



// Vector of each value of the gate Constraint
// ql * ai + qr * bi + qo * ci + qm * ai * bi + qc  = 0 // a single constraint for a gate.
// say a gate a * b = c; 
// you'd make it ql = 1, qr = 1, qo = -1, qm = 0, qc = 0, ai = a, bi = b, ci = c;
// making it a * b - c = 0;
// ai, bi, ci are the variables, the rest are constants.
// We create constraint for each gate in the circuit, then add the values to the vector. 
struct Constraint {
    ql: Vec<F>,
    qr: Vec<F>,
    qo: Vec<F>,
    qm: Vec<F>,
    qc: Vec<F>,
    ai: Vec<F>,
    bi: Vec<F>,
    ci: Vec<F>,
}

impl Constraint { 
    fn new() -> Self { 
        Constraint { 
            ql: Vec::new(),
            qr: Vec::new(),
            qo: Vec::new(),
            qm: Vec::new(),
            qc: Vec::new(),
            ai: Vec::new(),
            bi: Vec::new(),
            ci: Vec::new(),
        }
    }
    fn add_constraint(&mut self, ql: F, qr: F, qo: F, qm: F, qc: F, ai: F, bi: F, ci: F) { 
        self.ql.push(ql);
        self.qr.push(qr);
        self.qo.push(qo);
        self.qm.push(qm);
        self.qc.push(qc);
        self.ai.push(ai);
        self.bi.push(bi);
        self.ci.push(ci);
    }

    fn get_constraint(&self, index: usize) -> (F, F, F, F, F, F, F, F) { 
        (self.ql[index], self.qr[index], self.qo[index], self.qm[index], self.qc[index], self.ai[index], self.bi[index], self.ci[index])
    }

    fn get_gate_constraints(&self, index: usize) -> GateConstraints { 
       todo!();
    }
}


struct GateConstraints { 
    ql: Polynomial<F>,
    qr: Polynomial<F>,
    qo: Polynomial<F>,
    qm: Polynomial<F>,
    qc: Polynomial<F>,
    ax: Polynomial<F>,
    bx: Polynomial<F>,
    cx: Polynomial<F>,
}





