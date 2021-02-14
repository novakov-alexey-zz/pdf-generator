extern crate handlebars;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use handlebars::*;

pub fn array_length_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let length = h
        .param(0)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or_else(|| RenderError::new(
            "Param 0 with 'array' type is required for array_length helper",
        ))?;

    out.write(length.to_string().as_ref())?;

    Ok(())
}

// Check whether arrays have non-empty intersection based on given property name
//
// Example: (contains "category" ../master_data ["sensible"])
// return true: if array A (2nd parameter) contains any element from array B (3rd parameter).
//              1st parameter is property name to check at every element of the array A.
// return false: otherwise
pub fn contains_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let property = h.param(0)
        .as_ref()
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new(
            "Param 0 with 'string' type is required for 'contains' helper",
        ))?;

    let data: Vec<&str> = h
        .param(1)
        .as_ref()
        .and_then(|v| v.value()
            .as_array()
            .map(|v|
                v.iter()
                    .flat_map(|e|
                        e.as_object()
                            .and_then(|o| o.get(property)
                                .and_then(|vv| vv.as_str()))
                    )
                    .collect())
        )
        .ok_or_else(|| RenderError::new(
            "Param 1 with 'array' type is required for 'contains' helper",
        ))?;

    let keys: Vec<&str> = h
        .param(2)
        .as_ref()
        .and_then(|v| v.value().as_array()
            .map(|v| v.iter()
                .flat_map(|e| e.as_str())
                .collect())
        )
        .ok_or_else(|| RenderError::new(
            "Param 2 with 'array' type is required for 'contains' helper",
        ))?;

    let set: HashSet<&str> = HashSet::from_iter(data);
    let found = keys.iter().any(|k| set.contains(k));
    if found {
        out.write("true".to_string().as_ref())?
    }

    Ok(())
}

pub struct I18Helper(pub HashMap<String, String>);

impl HelperDef for I18Helper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let read_param = |i| h.param(i)
            .as_ref()
            .and_then(|v| v.value().as_str())
            .ok_or_else(|| RenderError::new(
                format!("Param {:?} with 'string' type is required for i18 helper", i),
            ));

        let key = read_param(0)?;

        let res = match self.0.get(key) {
            Some(v) => out.write(v),
            None => out.write(key)
        };

        res.map_err(|e|
            RenderError::new(format!("Failed to write into the Template output: {}",
                                     e.to_string())))
    }
}