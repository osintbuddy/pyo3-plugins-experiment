use pyo3::ffi::{PyObject, c_str};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::thread;
use std::time::Duration;

fn main() {
    let t1 = thread::spawn(move || {
        for _ in 1..5 {
            Python::with_gil(|py| {
                let locals = [("selenium", py.import("selenium").unwrap())].into_py_dict(py);
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
                Python::run(py, code, None, Some(&locals.unwrap())).unwrap();
            });
        }
    });
    let t2 = thread::spawn(move || {
        for _ in 1..5 {
            Python::with_gil(|py| {
                let locals = [("httpx", py.import("httpx").unwrap())]
                    .into_py_dict(py)
                    .unwrap();
                let code = c_str!("httpx.get('https://google.com')");
                let httpx_resp = py.eval(code, None, Some(&locals)).unwrap();

                println!("httpx test: {}", httpx_resp);
            });
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
