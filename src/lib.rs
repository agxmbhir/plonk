// eq - x^3 + x + 5 == 35
// Solution: x = 3;

use ark_ff::{FftField, Field};
use ark_poly::domain::general::GeneralElements;
use ark_poly::univariate::DensePolynomial as Polynomial;
use ark_poly::{EvaluationDomain, Evaluations, GeneralEvaluationDomain};
use ark_test_curves::bls12_381::Fq2 as F;

#[derive(Clone)]
struct Constraint<F: FftField> {
    ql: F,
    qr: F,
    qo: F,
    qm: F,
    qc: F,
    ai: F,
    bi: F,
    ci: F,
}

impl<F: FftField> Constraint<F> {
    fn new(ql: F, qr: F, qo: F, qm: F, qc: F, ai: F, bi: F, ci: F) -> Self {
        Constraint {
            ql,
            qr,
            qo,
            qm,
            qc,
            ai,
            bi,
            ci,
        }
    }
}
// Vector of each value of the gate Constraint
// ql * ai + qr * bi + qo * ci + qm * ai * bi + qc  = 0 // a single constraint for a gate.
// say a gate a * b = c;
// you'd make it ql = 1, qr = 1, qo = -1, qm = 0, qc = 0, ai = a, bi = b, ci = c;
// making it a * b - c = 0;
// ai, bi, ci are the variables, the rest are constants.
// We create constraint for each gate in the circuit, then add the values to the vector.
#[derive(Clone)]
struct Constraints<F: FftField> {
    constraints: Vec<Constraint<F>>,
    domain: GeneralEvaluationDomain<F>,
}

impl<F: FftField> Constraints<F> {
    fn new() -> Self {
        Constraints {
            constraints: Vec::new(),
            domain: GeneralEvaluationDomain::<F>::new(0).unwrap(),
        }
    }
    fn add_constraint(&mut self, constraint: &Constraint<F>) {
        self.constraints.push(constraint.clone());
        self.domain = GeneralEvaluationDomain::<F>::new(self.constraints.len()).unwrap();
    }

    fn get_constraint(&self, index: usize) -> &Constraint<F> {
        &self.constraints[index]
    }

    fn get_gate_constraints(&self, index: usize) -> &GateConstraints<F> {
        todo!()
    }
    // Returns the x values for each y value in the constraint.
    fn get_roots_of_unity(&self) -> GeneralElements<F> {
        self.domain.elements()
    }
}

// Final step, get constraints, constructed by interpolating the constraints of each gate.
struct GateConstraints<F: FftField> {
    ql: Polynomial<F>,
    qr: Polynomial<F>,
    qo: Polynomial<F>,
    qm: Polynomial<F>,
    qc: Polynomial<F>,
    ax: Polynomial<F>,
    bx: Polynomial<F>,
    cx: Polynomial<F>,
}

impl<F: FftField> GateConstraints<F> {
    // Construct a polynomial for each of the constraint vectors, assign them to the gate constraints.

    fn get_constraint_vecs(
        &self,
        c: &Constraints<F>,
    ) -> (
        Vec<F>,
        Vec<F>,
        Vec<F>,
        Vec<F>,
        Vec<F>,
        Vec<F>,
        Vec<F>,
        Vec<F>,
    ) {
        let mut ql: Vec<F> = Vec::new();
        let mut qr: Vec<F> = Vec::new();
        let mut qo: Vec<F> = Vec::new();
        let mut qm: Vec<F> = Vec::new();
        let mut qc: Vec<F> = Vec::new();
        let mut ax: Vec<F> = Vec::new();
        let mut bx: Vec<F> = Vec::new();
        let mut cx: Vec<F> = Vec::new();

        for constraint in c.constraints.iter() {
            ql.push(constraint.ql);
            qr.push(constraint.qr);
            qo.push(constraint.qo);
            qm.push(constraint.qm);
            qc.push(constraint.qc);
            ax.push(constraint.ai);
            bx.push(constraint.bi);
            cx.push(constraint.ci);
        }
        (ql, qr, qo, qm, qc, ax, bx, cx)
    }

    // Constructs evalutations for each of the constraint vectors.
    fn construct(&self, c: &Constraints<F>) -> Self {
        let (ql, qr, qo, qm, qc, ax, bx, cx) = self.get_constraint_vecs(c);

        let ql = Evaluations::from_vec_and_domain(ql, c.domain);
        let qr = Evaluations::from_vec_and_domain(qr, c.domain);
        let qo = Evaluations::from_vec_and_domain(qo, c.domain);
        let qm = Evaluations::from_vec_and_domain(qm, c.domain);
        let qc = Evaluations::from_vec_and_domain(qc, c.domain);
        let ax = Evaluations::from_vec_and_domain(ax, c.domain);
        let bx = Evaluations::from_vec_and_domain(bx, c.domain);
        let cx = Evaluations::from_vec_and_domain(cx, c.domain);
        GateConstraints {
            ql: ql.interpolate(),
            qr: qr.interpolate(),
            qo: qo.interpolate(),
            qm: qm.interpolate(),
            qc: qc.interpolate(),
            ax: ax.interpolate(),
            bx: bx.interpolate(),
            cx: cx.interpolate(),
        }
    }
}
