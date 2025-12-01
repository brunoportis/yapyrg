use pyo3::prelude::*;
use std::collections::HashMap;
use ignore::WalkBuilder;
use grep_regex::RegexMatcher;
use grep_searcher::{SearcherBuilder, BinaryDetection};
use grep_searcher::sinks::UTF8;

#[pyfunction]
fn search(py: Python, root_path: String, pattern: String) -> PyResult<Vec<HashMap<String, PyObject>>> {
    let mut results = Vec::new();
    
    let matcher = RegexMatcher::new(&pattern).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
    })?;
    
    let walker = WalkBuilder::new(root_path).build();
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .build();

    for result in walker {
        let entry = match result {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        
        if !entry.file_type().map_or(false, |ft| ft.is_file()) {
            continue;
        }

        let path = entry.path().to_owned();
        let path_str = path.to_string_lossy().to_string();
        
        let _ = searcher.search_path(
            &matcher,
            &path,
            UTF8(|lnum, line| {
                let mut match_data = HashMap::new();
                match_data.insert("path".to_string(), path_str.clone().into_py(py));
                match_data.insert("content".to_string(), line.to_string().into_py(py));
                match_data.insert("line".to_string(), lnum.into_py(py));
                
                results.push(match_data);
                Ok(true)
            }),
        );
    }

    Ok(results)
}

#[pymodule]
fn yapyrg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(search, m)?)?;
    Ok(())
}
