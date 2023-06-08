pub trait CollectResults {
    type OkValue;
    type ErrValue;

    fn collect_results(self) -> Result<Self::OkValue, Vec<Self::ErrValue>>;
}

pub trait CollectIterResults<T, Error> {
    fn collect_results(self) -> Result<Vec<T>, Vec<Error>>;
}

impl<Value, Error> CollectResults for Vec<Result<Value, Vec<Error>>> {
    type OkValue = Vec<Value>;
    type ErrValue = Error;

    fn collect_results(self) -> Result<Self::OkValue, Vec<Self::ErrValue>> {
        let mut errors = Vec::new();
        let mut ok_values = Vec::new();

        for result in self {
            match result {
                Ok(ok_value) => ok_values.push(ok_value),
                Err(errs) => errors.extend(errs),
            };
        }

        if errors.is_empty() {
            Ok(ok_values)
        } else {
            Err(errors)
        }
    }
}

impl<I, T, Error> CollectIterResults<T, Error> for I
where
    I: Iterator<Item = Result<T, Vec<Error>>>,
{
    fn collect_results(self) -> Result<Vec<T>, Vec<Error>> {
        let mut errors = Vec::new();
        let mut ok_values = Vec::new();

        self.for_each(|result| match result {
            Ok(ok_value) => ok_values.push(ok_value),
            Err(errs) => errors.extend(errs),
        });

        if errors.is_empty() {
            Ok(ok_values)
        } else {
            Err(errors)
        }
    }
}

impl<A, Error> CollectResults for (Result<A, Vec<Error>>,)
where
    A: Clone,
    Error: Clone,
{
    type OkValue = (A,);
    type ErrValue = Error;

    fn collect_results(self) -> Result<Self::OkValue, Vec<Self::ErrValue>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());

        let errors = vec![a_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(),))
    }
}

impl<A, B, Error> CollectResults for (Result<A, Vec<Error>>, Result<B, Vec<Error>>)
where
    A: Clone,
    B: Clone,
    Error: Clone,
{
    type OkValue = (A, B);
    type ErrValue = Error;

    fn collect_results(self) -> Result<Self::OkValue, Vec<Self::ErrValue>> {
        let (a, a_errs) = (self.0.clone().ok(), self.0.err().unwrap_or_default());
        let (b, b_errs) = (self.1.clone().ok(), self.1.err().unwrap_or_default());

        let errors = vec![a_errs, b_errs].concat();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok((a.unwrap(), b.unwrap()))
    }
}

impl<A, B, C, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    Error: Clone,
{
    type OkValue = (A, B, C);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C), Vec<Error>> {
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

impl<A, B, C, D, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    Error: Clone,
{
    type OkValue = (A, B, C, D);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D), Vec<Error>> {
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

impl<A, B, C, D, E, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
        Result<E, Vec<Error>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    Error: Clone,
{
    type OkValue = (A, B, C, D, E);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D, E), Vec<Error>> {
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

impl<A, B, C, D, E, F, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
        Result<E, Vec<Error>>,
        Result<F, Vec<Error>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    Error: Clone,
{
    type OkValue = (A, B, C, D, E, F);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D, E, F), Vec<Error>> {
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

impl<A, B, C, D, E, F, G, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
        Result<E, Vec<Error>>,
        Result<F, Vec<Error>>,
        Result<G, Vec<Error>>,
    )
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
    G: Clone,
    Error: Clone,
{
    type OkValue = (A, B, C, D, E, F, G);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D, E, F, G), Vec<Error>> {
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

impl<A, B, C, D, E, F, G, H, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
        Result<E, Vec<Error>>,
        Result<F, Vec<Error>>,
        Result<G, Vec<Error>>,
        Result<H, Vec<Error>>,
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
    Error: Clone,
{
    type OkValue = (A, B, C, D, E, F, G, H);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D, E, F, G, H), Vec<Error>> {
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

impl<A, B, C, D, E, F, G, H, I, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
        Result<E, Vec<Error>>,
        Result<F, Vec<Error>>,
        Result<G, Vec<Error>>,
        Result<H, Vec<Error>>,
        Result<I, Vec<Error>>,
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
    Error: Clone,
{
    type OkValue = (A, B, C, D, E, F, G, H, I);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D, E, F, G, H, I), Vec<Error>> {
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

impl<A, B, C, D, E, F, G, H, I, J, Error> CollectResults
    for (
        Result<A, Vec<Error>>,
        Result<B, Vec<Error>>,
        Result<C, Vec<Error>>,
        Result<D, Vec<Error>>,
        Result<E, Vec<Error>>,
        Result<F, Vec<Error>>,
        Result<G, Vec<Error>>,
        Result<H, Vec<Error>>,
        Result<I, Vec<Error>>,
        Result<J, Vec<Error>>,
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
    Error: Clone,
{
    type OkValue = (A, B, C, D, E, F, G, H, I, J);
    type ErrValue = Error;

    fn collect_results(self) -> Result<(A, B, C, D, E, F, G, H, I, J), Vec<Error>> {
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
