// Copyright 2015 Danny Guo
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

macro_rules! assert_approx_eq_float {
    ( $expected:expr, $actual:expr, $accuracy:expr ) => {
        if ($expected - $actual).abs() >= $accuracy {
            panic!("\
assertion failed: `actual not approximately equal`
   actual: `{}`,
 expected: `{}`: actual not within < {} of expected",
                $actual, $expected, $accuracy);
        }
    }
}
