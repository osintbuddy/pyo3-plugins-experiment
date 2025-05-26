use pyo3::{prelude::*};
use pyo3::types::{IntoPyDict};
use pyo3::ffi::c_str;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let locals = [("selenium", py.import("selenium")?)].into_py_dict(py)?;
        let code = c_str!(r#"
from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.by import By
driver = webdriver.Firefox()
driver.get('http://www.python.org')
assert 'Python' in driver.title
elm = driver.find_element(By.NAME, 'q')
print(elm)
    "#);
        Python::run(py, code, None, Some(&locals))?;

        let locals = [("httpx", py.import("httpx")?)].into_py_dict(py)?;
        let code = c_str!("httpx.get('https://google.com')");
        let httpx_resp: PyObject = py.eval(code, None, Some(&locals))?.extract()?;

        println!("httpx test: {}", httpx_resp);
        Ok(())
    })
}