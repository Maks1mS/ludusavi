//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait CLLocationManagerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "CoreLocation_CLLocationManager"
        ))]
        #[deprecated = "Implement -locationManager:didUpdateLocations: instead"]
        #[optional]
        #[method(locationManager:didUpdateToLocation:fromLocation:)]
        unsafe fn locationManager_didUpdateToLocation_fromLocation(
            &self,
            manager: &CLLocationManager,
            new_location: &CLLocation,
            old_location: &CLLocation,
        );

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(locationManager:didUpdateLocations:)]
        unsafe fn locationManager_didUpdateLocations(
            &self,
            manager: &CLLocationManager,
            locations: &NSArray<CLLocation>,
        );

        #[cfg(all(
            feature = "CoreLocation_CLHeading",
            feature = "CoreLocation_CLLocationManager"
        ))]
        #[optional]
        #[method(locationManager:didUpdateHeading:)]
        unsafe fn locationManager_didUpdateHeading(
            &self,
            manager: &CLLocationManager,
            new_heading: &CLHeading,
        );

        #[cfg(feature = "CoreLocation_CLLocationManager")]
        #[optional]
        #[method(locationManagerShouldDisplayHeadingCalibration:)]
        unsafe fn locationManagerShouldDisplayHeadingCalibration(
            &self,
            manager: &CLLocationManager,
        ) -> bool;

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "CoreLocation_CLRegion"
        ))]
        #[optional]
        #[method(locationManager:didDetermineState:forRegion:)]
        unsafe fn locationManager_didDetermineState_forRegion(
            &self,
            manager: &CLLocationManager,
            state: CLRegionState,
            region: &CLRegion,
        );

        #[cfg(all(
            feature = "CoreLocation_CLBeacon",
            feature = "CoreLocation_CLBeaconRegion",
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSArray"
        ))]
        #[deprecated]
        #[optional]
        #[method(locationManager:didRangeBeacons:inRegion:)]
        unsafe fn locationManager_didRangeBeacons_inRegion(
            &self,
            manager: &CLLocationManager,
            beacons: &NSArray<CLBeacon>,
            region: &CLBeaconRegion,
        );

        #[cfg(all(
            feature = "CoreLocation_CLBeaconRegion",
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSError"
        ))]
        #[deprecated]
        #[optional]
        #[method(locationManager:rangingBeaconsDidFailForRegion:withError:)]
        unsafe fn locationManager_rangingBeaconsDidFailForRegion_withError(
            &self,
            manager: &CLLocationManager,
            region: &CLBeaconRegion,
            error: &NSError,
        );

        #[cfg(all(
            feature = "CoreLocation_CLBeacon",
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(locationManager:didRangeBeacons:satisfyingConstraint:)]
        unsafe fn locationManager_didRangeBeacons_satisfyingConstraint(
            &self,
            manager: &CLLocationManager,
            beacons: &NSArray<CLBeacon>,
            beacon_constraint: &CLBeaconIdentityConstraint,
        );

        #[cfg(all(
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(locationManager:didFailRangingBeaconsForConstraint:error:)]
        unsafe fn locationManager_didFailRangingBeaconsForConstraint_error(
            &self,
            manager: &CLLocationManager,
            beacon_constraint: &CLBeaconIdentityConstraint,
            error: &NSError,
        );

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "CoreLocation_CLRegion"
        ))]
        #[optional]
        #[method(locationManager:didEnterRegion:)]
        unsafe fn locationManager_didEnterRegion(
            &self,
            manager: &CLLocationManager,
            region: &CLRegion,
        );

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "CoreLocation_CLRegion"
        ))]
        #[optional]
        #[method(locationManager:didExitRegion:)]
        unsafe fn locationManager_didExitRegion(
            &self,
            manager: &CLLocationManager,
            region: &CLRegion,
        );

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(locationManager:didFailWithError:)]
        unsafe fn locationManager_didFailWithError(
            &self,
            manager: &CLLocationManager,
            error: &NSError,
        );

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "CoreLocation_CLRegion",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(locationManager:monitoringDidFailForRegion:withError:)]
        unsafe fn locationManager_monitoringDidFailForRegion_withError(
            &self,
            manager: &CLLocationManager,
            region: Option<&CLRegion>,
            error: &NSError,
        );

        #[cfg(feature = "CoreLocation_CLLocationManager")]
        #[deprecated]
        #[optional]
        #[method(locationManager:didChangeAuthorizationStatus:)]
        unsafe fn locationManager_didChangeAuthorizationStatus(
            &self,
            manager: &CLLocationManager,
            status: CLAuthorizationStatus,
        );

        #[cfg(feature = "CoreLocation_CLLocationManager")]
        #[optional]
        #[method(locationManagerDidChangeAuthorization:)]
        unsafe fn locationManagerDidChangeAuthorization(&self, manager: &CLLocationManager);

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "CoreLocation_CLRegion"
        ))]
        #[optional]
        #[method(locationManager:didStartMonitoringForRegion:)]
        unsafe fn locationManager_didStartMonitoringForRegion(
            &self,
            manager: &CLLocationManager,
            region: &CLRegion,
        );

        #[cfg(feature = "CoreLocation_CLLocationManager")]
        #[optional]
        #[method(locationManagerDidPauseLocationUpdates:)]
        unsafe fn locationManagerDidPauseLocationUpdates(&self, manager: &CLLocationManager);

        #[cfg(feature = "CoreLocation_CLLocationManager")]
        #[optional]
        #[method(locationManagerDidResumeLocationUpdates:)]
        unsafe fn locationManagerDidResumeLocationUpdates(&self, manager: &CLLocationManager);

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(locationManager:didFinishDeferredUpdatesWithError:)]
        unsafe fn locationManager_didFinishDeferredUpdatesWithError(
            &self,
            manager: &CLLocationManager,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "CoreLocation_CLLocationManager",
            feature = "CoreLocation_CLVisit"
        ))]
        #[optional]
        #[method(locationManager:didVisit:)]
        unsafe fn locationManager_didVisit(&self, manager: &CLLocationManager, visit: &CLVisit);
    }

    unsafe impl ProtocolType for dyn CLLocationManagerDelegate {}
);
