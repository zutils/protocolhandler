#![allow(non_snake_case)]

use crate::{Destination, RpcData, Transport, VecModuleInfo, VecRpcData};
use crate::propagator::Propagator;
use crate::common::CommonModule;
use crate::transportresponse::{TransportResponse, TransportCombiner};

use crate::transport_autogen::transport::{DataType, RequestType, DataType_oneof_result};

use failure::Error;

// Anywhere there is a common module, we want transports working with them.
impl<T> TransportToModuleGlue for T where T: CommonModule {}

/// These functions are the endpoints to the different modules.
pub trait TransportToModuleGlue: CommonModule {
    fn handle_transport(&self, transport: &Transport) -> Result<Vec<Transport>, Error> {
        // Pass on transport to proper function
        match transport.request_type {
            RequestType::GET_INFO => self.get_info_glue(transport),
            RequestType::RECEIVE_RPC_AS_CLIENT => self.receive_rpc_as_client_glue(transport),
            RequestType::RECEIVE_RPC_AS_SERVER => self.receive_rpc_as_server_glue(transport),
            RequestType::RECEIVE_PUBLIC_RPC => self.receive_public_rpc_glue(transport),
            RequestType::NONE => { log::error!("No request type to handle!"); Ok(Vec::new()) },
        }
    }

    fn get_info_glue(&self, transport: &Transport) -> Result<Vec<Transport>, Error> { 
        let msg = transport.get_payload().get_destination();
        let module_ret = self.get_info(msg)?;
        let result = DataType_oneof_result::vecmoduleinfo(module_ret);
        let ret = TransportResponse::create_Transport_result(result);
        Ok(vec![ret])
    }

    fn receive_rpc_as_client_glue(&self, transport: &Transport) -> Result<Vec<Transport>, Error> { 
        let msg = transport.get_payload().get_rpcdata();
        let module_ret = self.receive_rpc_as_client(msg)?;
        let result = DataType_oneof_result::vecrpcdata(module_ret);
        let ret = TransportResponse::create_Transport_result(result);
        Ok(vec![ret])
    }

    fn receive_rpc_as_server_glue(&self, transport: &Transport) -> Result<Vec<Transport>, Error> { 
        let msg = transport.get_payload().get_rpcdata();
        let module_ret = self.receive_rpc_as_server(msg)?;
        let result = DataType_oneof_result::vecrpcdata(module_ret);
        let ret = TransportResponse::create_Transport_result(result);
        Ok(vec![ret])
    }

    fn receive_public_rpc_glue(&self, transport: &Transport) -> Result<Vec<Transport>, Error> { 
        let msg = transport.get_payload().get_rpcdata();
        let module_ret = self.receive_public_rpc(msg)?;
        let result = DataType_oneof_result::vecrpcdata(module_ret);
        let ret = TransportResponse::create_Transport_result(result);
        Ok(vec![ret])
    }
}

/// This is glue to package up requests into Transports and unpacking them.
pub trait ModuleToTransportGlue: Propagator {
    fn get_info(&self, data: Destination) -> Result<VecModuleInfo, Error> {
        log::debug!("Propagating get_info({:?})", data);
        let transport = TransportRequest::create_GET_INFO(data);
        let transport_results = self.propagate_transport(&transport);
        TransportCombiner::combine_to_VecModuleInfo(transport_results)
    }

    fn receive_rpc_as_client(&self, data: RpcData) -> Result<VecRpcData, Error> {
        log::debug!("Propagating receive_rpc_as_client({:?})", data);
        let transport = TransportRequest::create_RECEIVE_RPC_AS_CLIENT(data);
        let transport_results = self.propagate_transport(&transport);
        TransportCombiner::combine_to_VecRpcData(transport_results)
    }

    fn receive_rpc_as_server(&self, data: RpcData) -> Result<VecRpcData, Error> {
        log::debug!("Propagating receive_rpc_as_server({:?})", data);
        let transport = TransportRequest::create_RECEIVE_RPC_AS_SERVER(data);
        let transport_results = self.propagate_transport(&transport);
        TransportCombiner::combine_to_VecRpcData(transport_results)
    }

    fn receive_public_rpc(&self, data: RpcData) -> Result<VecRpcData, Error> {
        log::debug!("Propagating receive_public_rpc({:?})", data);
        let transport = TransportRequest::create_RECEIVE_PUBLIC_RPC(data);
        let transport_results = self.propagate_transport(&transport);
        TransportCombiner::combine_to_VecRpcData(transport_results)
    }
}


pub struct TransportRequest;
impl TransportRequest {
    fn create_Transport_result(data: DataType_oneof_result) -> Transport {
        let mut data_type = DataType::new();
        data_type.result = Some(data);

        let mut ret = Transport::default();
        ret.set_payload(data_type);
        ret
    }

    pub fn create_GET_INFO(data: Destination) -> Transport {
        let destination = data.get_schema().clone();
        let result = DataType_oneof_result::destination(data);
        let mut transport = TransportRequest::create_Transport_result(result);
        transport.set_request_type(RequestType::GET_INFO);
        transport.set_destination(destination);
        transport
    }

    pub fn create_RECEIVE_RPC_AS_CLIENT(data: RpcData) -> Transport {
        let destination = data.get_schema().clone();
        let result = DataType_oneof_result::rpcdata(data);
        let mut transport = TransportRequest::create_Transport_result(result);
        transport.set_request_type(RequestType::RECEIVE_RPC_AS_CLIENT);
        transport.set_destination(destination);
        transport
    }

    pub fn create_RECEIVE_RPC_AS_SERVER(data: RpcData) -> Transport {
        let destination = data.get_schema().clone();
        let result = DataType_oneof_result::rpcdata(data);
        let mut transport = TransportRequest::create_Transport_result(result);
        transport.set_request_type(RequestType::RECEIVE_RPC_AS_SERVER);
        transport.set_destination(destination);
        transport
    }

    pub fn create_RECEIVE_PUBLIC_RPC(data: RpcData) -> Transport {
        let destination = data.get_schema().clone();
        let result = DataType_oneof_result::rpcdata(data);
        let mut transport = TransportRequest::create_Transport_result(result);
        transport.set_request_type(RequestType::RECEIVE_PUBLIC_RPC);
        transport.set_destination(destination);
        transport
    }
}
