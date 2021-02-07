use crate::{NAN, PI};
// use std::f64::

// def _Byx(self, x, y):
//     """ By using 2D Model for rectangular sheets magnetised in x-plane

//     Args:
//         magnet ([type]): [magnet object]
//         x ([type]): [x array]
//         y ([type]): [y array]

//     Returns:
//         [array]: [By]
//     """
//     a = self.a
//     b = self.b
//     J = self.Jx
//     return (-J / (4 * PI)) * (
//         _np.log(((x - a)**2 + (y - b)**2) / ((x + a)**2 + (y - b)**2))
//         - _np.log(((x - a)**2 + (y + b)**2) / ((x + a)**2 + (y + b)**2))
//     )

// def _Bxy(self, x, y):
//     """ Bx using 2D Model for rectangular sheets magnetised in y-plane

//     Args:
//         magnet ([type]): [magnet object]
//         x ([type]): [x array]
//         y ([type]): [y array]

//     Returns:
//         [array]: [Bx]
//     """
//     a = self.a
//     b = self.b
//     J = self.Jy
//     return (J / (4 * PI)) * (
//         _np.log(((x + a)**2 + (y - b)**2) / ((x + a)**2 + (y + b)**2))
//         - _np.log(((x - a)**2 + (y - b)**2) / ((x - a)**2 + (y + b)**2))
//     )

// def _Byy(self, x, y):
//     """ By using 2D Model for rectangular sheets magnetised in y-plane

//     Args:
//         magnet ([type]): [magnet object]
//         x ([type]): [x array]
//         y ([type]): [y array]

//     Returns:
//         [array]: [By]
//     """
//     a = self.a
//     b = self.b
//     J = self.Jy
//     return (J / (2 * PI)) * (
//         _np.arctan2((2 * b * (x + a)), ((x + a)**2 + y**2 - b**2))
//         - _np.arctan2((2 * b * (x - a)), ((x - a)**2 + y**2 - b**2))
//     )
