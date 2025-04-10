// Code generated by lister-gen. DO NOT EDIT.

package v1

import (
	managementv1 "github.com/loft-sh/api/v4/pkg/apis/management/v1"
	labels "k8s.io/apimachinery/pkg/labels"
	listers "k8s.io/client-go/listers"
	cache "k8s.io/client-go/tools/cache"
)

// AgentAuditEventLister helps list AgentAuditEvents.
// All objects returned here must be treated as read-only.
type AgentAuditEventLister interface {
	// List lists all AgentAuditEvents in the indexer.
	// Objects returned here must be treated as read-only.
	List(selector labels.Selector) (ret []*managementv1.AgentAuditEvent, err error)
	// Get retrieves the AgentAuditEvent from the index for a given name.
	// Objects returned here must be treated as read-only.
	Get(name string) (*managementv1.AgentAuditEvent, error)
	AgentAuditEventListerExpansion
}

// agentAuditEventLister implements the AgentAuditEventLister interface.
type agentAuditEventLister struct {
	listers.ResourceIndexer[*managementv1.AgentAuditEvent]
}

// NewAgentAuditEventLister returns a new AgentAuditEventLister.
func NewAgentAuditEventLister(indexer cache.Indexer) AgentAuditEventLister {
	return &agentAuditEventLister{listers.New[*managementv1.AgentAuditEvent](indexer, managementv1.Resource("agentauditevent"))}
}
