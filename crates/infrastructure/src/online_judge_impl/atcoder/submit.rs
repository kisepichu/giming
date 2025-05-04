use usecases::service_error::ServiceError;

use crate::{detail_error::DetailError, external::atcoder_requester::AtcoderRequester};

use super::Atcoder;

impl<R: AtcoderRequester> Atcoder<R> {
    pub fn submit(&self, _solution_id: String) -> Result<(), ServiceError<DetailError>> {
        todo!()
    }
}
