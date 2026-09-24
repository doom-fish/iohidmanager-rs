use super::{
    DeviceRemovalSubscription, HidDevice, HidDeviceId, HidDeviceInfo, HidElement, HidInputReport,
    HidManager, HidQueue, HidTransaction, HidValue, HidValueInfo, ManagerDeviceSubscription,
    ManagerReportSubscription, ManagerValueSubscription, QueueValueAvailableSubscription,
    ReportSubscription, TimestampedReportSubscription, ValueSubscription,
};
use crate::error::HidError;

const _: fn() = || {
    trait AmbiguousIfSend<A> {
        fn check() {}
    }

    impl<T: ?Sized> AmbiguousIfSend<()> for T {}
    impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}

    trait AmbiguousIfSync<A> {
        fn check() {}
    }

    impl<T: ?Sized> AmbiguousIfSync<()> for T {}
    impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

    <HidManager as AmbiguousIfSend<_>>::check();
    <HidManager as AmbiguousIfSync<_>>::check();
    <HidDevice as AmbiguousIfSend<_>>::check();
    <HidDevice as AmbiguousIfSync<_>>::check();
    <HidElement as AmbiguousIfSend<_>>::check();
    <HidElement as AmbiguousIfSync<_>>::check();
    <HidValue as AmbiguousIfSend<_>>::check();
    <HidValue as AmbiguousIfSync<_>>::check();
    <HidQueue as AmbiguousIfSend<_>>::check();
    <HidQueue as AmbiguousIfSync<_>>::check();
    <HidTransaction as AmbiguousIfSend<_>>::check();
    <HidTransaction as AmbiguousIfSync<_>>::check();
};

const _: () = {
    const fn shareable<T: Send + Sync>() {}

    shareable::<ReportSubscription>();
    shareable::<TimestampedReportSubscription>();
    shareable::<ValueSubscription>();
    shareable::<DeviceRemovalSubscription>();
    shareable::<ManagerDeviceSubscription>();
    shareable::<ManagerReportSubscription>();
    shareable::<ManagerValueSubscription>();
    shareable::<QueueValueAvailableSubscription>();
    shareable::<HidDeviceId>();
    shareable::<HidDeviceInfo>();
    shareable::<HidValueInfo>();
    shareable::<HidInputReport>();
    shareable::<HidError>();
};

#[cfg(feature = "async")]
const _: () = {
    use crate::async_api::{
        DeviceInputValueStream, DeviceMatchingEvent, DeviceRemovalEvent, DeviceRemovalStream,
        InputReportEvent, InputValueEvent, ManagerDeviceMatchingStream, ManagerDeviceRemovalStream,
        ManagerInputReportStream, ManagerInputValueStream, QueueValueStream,
    };

    const fn shareable<T: Send + Sync>() {}

    shareable::<ManagerInputValueStream>();
    shareable::<ManagerDeviceMatchingStream>();
    shareable::<ManagerDeviceRemovalStream>();
    shareable::<ManagerInputReportStream>();
    shareable::<DeviceRemovalStream>();
    shareable::<DeviceInputValueStream>();
    shareable::<QueueValueStream>();
    shareable::<InputValueEvent>();
    shareable::<DeviceMatchingEvent>();
    shareable::<DeviceRemovalEvent>();
    shareable::<InputReportEvent>();
};
