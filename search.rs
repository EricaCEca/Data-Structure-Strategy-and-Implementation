use std::collections::{HashMap, HashSet};

impl SearchEngine {
    pub fn search(&self, query: &str, top_k: usize, mode: &str) -> Vec<&Product> {
        let tokens = tokenize(query);
        if tokens.is_empty() {
            return vec![];
        }

        match mode {
            // ----------------------------
            // Modo OR: união dos resultados
            // ----------------------------
            "OR" => {
                let mut results_set: HashSet<u64> = HashSet::new();

                for t in tokens {
                    if let Some(pst) = self.index.get_postings(&t) {
                        results_set.extend(pst.iter().copied());
                    }
                }

                let mut results: Vec<&Product> = results_set
                    .into_iter()
                    .filter_map(|id| self.products.get(&id))
                    .collect();

                results.sort_by_key(|p| p.id);
                results.truncate(top_k);
                results
            }

            // ----------------------------
            // Modo AND: interseção dos resultados
            // ----------------------------
            "AND" | _ => {
                let mut results_set: Option<HashSet<u64>> = None;

                for t in tokens {
                    if let Some(pst) = self.index.get_postings(&t) {
                        if let Some(curr) = &mut results_set {
                            let intersect: HashSet<u64> =
                                curr.intersection(pst).copied().collect();
                            *curr = intersect;
                        } else {
                            results_set = Some(pst.clone());
                        }
                    } else {
                        // token não existe => resultado vazio
                        return vec![];
                    }
                }

                let mut results: Vec<&Product> = match results_set {
                    Some(set) => set.into_iter().filter_map(|id| self.products.get(&id)).collect(),
                    None => vec![],
