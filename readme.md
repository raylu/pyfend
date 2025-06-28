python bindings for [fend-core](https://crates.io/crates/fend-core)

## example
```
>>> import pyfend
>>> context = pyfend.Context()

>>> pyfend.evaluate('a = pi to 3 sf', context)
'approx. 3.14'
>>> pyfend.evaluate('a mi to ft', context)
'approx. 16500 ft'

>>> pyfend.evaluate('2d4', context)
'{ 2: 6.25%, 3: 12.50%, 4: 18.75%, 5: 25.00%, 6: 18.75%, 7: 12.50%, 8: 6.25% }'
>>> context.set_output_mode_terminal()
>>> print(pyfend.evaluate('2d4', context))
  2:  6.25%  #######
  3: 12.50%  ###############
  4: 18.75%  ######################
  5: 25.00%  ##############################
  6: 18.75%  ######################
  7: 12.50%  ###############
  8:  6.25%  #######

>>> pyfend.evaluate('@debug 1kg to m', context)
pyfend.FendError: cannot convert from kg to m: units 'kilogram' and 'meter' are incompatible
```

## testing
do not run `uv -m unittest`. bypass the uv cache:
```
uv run maturin develop --uv && .venv/bin/python -m unittest
```
