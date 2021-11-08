use crate::props::props::IProps;
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) trait IPropsProvider<PR> where PR: IProps {
    fn get_version(&self) -> VersionId;
    fn get_props(&self, _period: &dyn IPeriod) -> PR;
}