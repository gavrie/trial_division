from trial_division import trial_division

def test_trial_division():
    assert trial_division(123456789) == [3, 3, 3607, 3803]
