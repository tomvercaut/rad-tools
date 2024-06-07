# bio-dose

## Description

A library and command line interfaces (CLIs) to compute EQD2 and BED doses.

### EQD2

EQD2 is the dose in 2 Gy fractions, which is biologically equivalent to the total dose ($D$) delivered in $d$ fraction doses.
EQD2 is based on the linear quadratic model of cell survival.

```math
  EQD2=D \left( \frac{d + \frac{\alpha}{\beta}}{2 + \frac{\alpha}{\beta}} \right)
```

### BED

BED is the Biologically Effective Dose for tissue with a (well) defined $\frac{\alpha}{\beta}$ ratio.

```math
BED=nd\left( 1 + \frac{d}{\frac{\alpha}{\beta}} \right)
```

BED - LQ model with time factor

```math
BED=nd\left( 1 + \frac{d}{\frac{\alpha}{\beta}} \right) 
             - \frac{log_{e}(2)(T-T_{k})}{\alpha T_{p}}
```

## Symbols

* $n$ number of fractions
* $d$ dose per fraction in Gy
* $\alpha$ and $\beta$ linear and quadratic coefficients in the LQ model.
  $\frac{\alpha}{\beta}$ corresponds to the dose (Gy) at which the lineair
  and quadratic compoment of cell kill are equal.
* $T$ overall time of treatment in days
* $T_{p}$ cell doubling time
* $T_{k}$ day on which cell repopulation starts

## Bibliography

[1] The linear-quadratic formula and progress in fractionated radiotherapy.
  British Journal of Radiology, Volume 62, Issue 740, 1 August 1989, Pages 679–694
  [https://doi.org/10.1259/0007-1285-62-740-679](https://doi.org/10.1259/0007-1285-62-740-679)
[2] 21 years of Biologically Effective Dose.
  British Journal of Radiology, Volume 83, Issue 991, 1 July 2010, Pages 554–568,
  [https://doi.org/10.1259/bjr/31372149](https://doi.org/10.1259/bjr/31372149)
