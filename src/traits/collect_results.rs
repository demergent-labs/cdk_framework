pub trait CollectResults<T, X> {
    fn collect_results(self) -> Result<T, Vec<X>>;
}

impl<A, X> CollectResults<(A,), X> for (Result<A, Vec<X>>,)
where
    A: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A,), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());

        let errors = vec![a_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(),))
    }
}

impl<A, B, X> CollectResults<(A, B), X> for (Result<A, Vec<X>>, Result<B, Vec<X>>)
where
    A: Clone,
    B: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(), b.unwrap()))
    }
}

impl<A, B, C, X> CollectResults<(A, B, C), X>
    for (Result<A, Vec<X>>, Result<B, Vec<X>>, Result<C, Vec<X>>)
where
    A: Clone,
    B: Clone,
    C: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs, c_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(), b.unwrap(), c.unwrap()))
    }
}

impl<A, B, C, D, X> CollectResults<(A, B, C, D), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs, c_errs, d_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap()))
    }
}

impl<A, B, C, D, E, X> CollectResults<(A, B, C, D, E), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
        Result<E, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D, E), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());
        let (e, e_errs) = (self.4.clone().ok(), self.4.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs, c_errs, d_errs, e_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap(), e.unwrap()))
    }
}

impl<A, B, C, D, E, F, X> CollectResults<(A, B, C, D, E, F), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
        Result<E, Vec<X>>,
        Result<F, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D, E, F), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());
        let (e, e_errs) = (self.4.clone().ok(), self.4.err().unwrap_or_default());
        let (f, f_errs) = (self.5.clone().ok(), self.5.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs, c_errs, d_errs, e_errs, f_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
            e.unwrap(),
            f.unwrap(),
        ))
    }
}

impl<A, B, C, D, E, F, G, X> CollectResults<(A, B, C, D, E, F, G), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
        Result<E, Vec<X>>,
        Result<F, Vec<X>>,
        Result<G, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    G: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D, E, F, G), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());
        let (e, e_errs) = (self.4.clone().ok(), self.4.err().unwrap_or_default());
        let (f, f_errs) = (self.5.clone().ok(), self.5.err().unwrap_or_default());
        let (g, g_errs) = (self.6.clone().ok(), self.6.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs, c_errs, d_errs, e_errs, f_errs, g_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
            e.unwrap(),
            f.unwrap(),
            g.unwrap(),
        ))
    }
}

impl<A, B, C, D, E, F, G, H, X> CollectResults<(A, B, C, D, E, F, G, H), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
        Result<E, Vec<X>>,
        Result<F, Vec<X>>,
        Result<G, Vec<X>>,
        Result<H, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    G: Clone,
    H: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D, E, F, G, H), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());
        let (e, e_errs) = (self.4.clone().ok(), self.4.err().unwrap_or_default());
        let (f, f_errs) = (self.5.clone().ok(), self.5.err().unwrap_or_default());
        let (g, g_errs) = (self.6.clone().ok(), self.6.err().unwrap_or_default());
        let (h, h_errs) = (self.7.clone().ok(), self.7.err().unwrap_or_default());

        let errors = vec![
            a_errs, b_errs, c_errs, d_errs, e_errs, f_errs, g_errs, h_errs,
        ]
        .concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
            e.unwrap(),
            f.unwrap(),
            g.unwrap(),
            h.unwrap(),
        ))
    }
}

impl<A, B, C, D, E, F, G, H, I, X> CollectResults<(A, B, C, D, E, F, G, H, I), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
        Result<E, Vec<X>>,
        Result<F, Vec<X>>,
        Result<G, Vec<X>>,
        Result<H, Vec<X>>,
        Result<I, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    G: Clone,
    H: Clone,
    I: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D, E, F, G, H, I), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());
        let (e, e_errs) = (self.4.clone().ok(), self.4.err().unwrap_or_default());
        let (f, f_errs) = (self.5.clone().ok(), self.5.err().unwrap_or_default());
        let (g, g_errs) = (self.6.clone().ok(), self.6.err().unwrap_or_default());
        let (h, h_errs) = (self.7.clone().ok(), self.7.err().unwrap_or_default());
        let (i, i_errs) = (self.8.clone().ok(), self.8.err().unwrap_or_default());

        let errors = vec![
            a_errs, b_errs, c_errs, d_errs, e_errs, f_errs, g_errs, h_errs, i_errs,
        ]
        .concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
            e.unwrap(),
            f.unwrap(),
            g.unwrap(),
            h.unwrap(),
            i.unwrap(),
        ))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, X> CollectResults<(A, B, C, D, E, F, G, H, I, J), X>
    for (
        Result<A, Vec<X>>,
        Result<B, Vec<X>>,
        Result<C, Vec<X>>,
        Result<D, Vec<X>>,
        Result<E, Vec<X>>,
        Result<F, Vec<X>>,
        Result<G, Vec<X>>,
        Result<H, Vec<X>>,
        Result<I, Vec<X>>,
        Result<J, Vec<X>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    G: Clone,
    H: Clone,
    I: Clone,
    J: Clone,
    X: Clone,
{
    fn collect_results(self) -> Result<(A, B, C, D, E, F, G, H, I, J), Vec<X>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());
        let (c, c_errs) = (self.2.clone().ok(), self.2.err().unwrap_or_default());
        let (d, d_errs) = (self.3.clone().ok(), self.3.err().unwrap_or_default());
        let (e, e_errs) = (self.4.clone().ok(), self.4.err().unwrap_or_default());
        let (f, f_errs) = (self.5.clone().ok(), self.5.err().unwrap_or_default());
        let (g, g_errs) = (self.6.clone().ok(), self.6.err().unwrap_or_default());
        let (h, h_errs) = (self.7.clone().ok(), self.7.err().unwrap_or_default());
        let (i, i_errs) = (self.8.clone().ok(), self.8.err().unwrap_or_default());
        let (j, j_errs) = (self.9.clone().ok(), self.9.err().unwrap_or_default());

        let errors = vec![
            a_errs, b_errs, c_errs, d_errs, e_errs, f_errs, g_errs, h_errs, i_errs, j_errs,
        ]
        .concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
            e.unwrap(),
            f.unwrap(),
            g.unwrap(),
            h.unwrap(),
            i.unwrap(),
            j.unwrap(),
        ))
    }
}
