// Code generated by lister-gen. DO NOT EDIT.

package v1

import (
	managementv1 "github.com/loft-sh/api/v4/pkg/apis/management/v1"
	labels "k8s.io/apimachinery/pkg/labels"
	listers "k8s.io/client-go/listers"
	cache "k8s.io/client-go/tools/cache"
)

// RegisterVirtualClusterLister helps list RegisterVirtualClusters.
// All objects returned here must be treated as read-only.
type RegisterVirtualClusterLister interface {
	// List lists all RegisterVirtualClusters in the indexer.
	// Objects returned here must be treated as read-only.
	List(selector labels.Selector) (ret []*managementv1.RegisterVirtualCluster, err error)
	// Get retrieves the RegisterVirtualCluster from the index for a given name.
	// Objects returned here must be treated as read-only.
	Get(name string) (*managementv1.RegisterVirtualCluster, error)
	RegisterVirtualClusterListerExpansion
}

// registerVirtualClusterLister implements the RegisterVirtualClusterLister interface.
type registerVirtualClusterLister struct {
	listers.ResourceIndexer[*managementv1.RegisterVirtualCluster]
}

// NewRegisterVirtualClusterLister returns a new RegisterVirtualClusterLister.
func NewRegisterVirtualClusterLister(indexer cache.Indexer) RegisterVirtualClusterLister {
	return &registerVirtualClusterLister{listers.New[*managementv1.RegisterVirtualCluster](indexer, managementv1.Resource("registervirtualcluster"))}
}
