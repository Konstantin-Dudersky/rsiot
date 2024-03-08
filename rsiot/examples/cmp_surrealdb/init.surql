DEFINE TABLE raw;

DEFINE FIELD entity ON TABLE raw TYPE string;
DEFINE FIELD attribute ON TABLE raw TYPE option<string>;
DEFINE FIELD value_int ON TABLE raw TYPE option<int>;
DEFINE FIELD value_float ON TABLE raw TYPE option<float>;
DEFINE FIELD value_bool ON TABLE raw TYPE option<bool>;
DEFINE FIELD value_string ON TABLE raw TYPE option<string>;
