# rust-test-validator


It is validate with message, rules and ValidationResult <br/>


Example below: <br/> 
<pre>
let rules_hash: HashMap<&str, Vec<Rules>> = rule_map![<br/>
     "ros" => vec![Rules::In {value: "123,asd,123".to_string()}],<br/>
      "ross" => vec![Rules::GreaterThan {field:"as".to_string()}]<br/>
   ];</pre>
